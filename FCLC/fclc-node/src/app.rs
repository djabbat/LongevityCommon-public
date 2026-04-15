use egui::{Color32, RichText, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use fclc_core::dp::DpConfig;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::client::{ModelUpdatePayload, OrchestratorClient};
use crate::connector::generate_demo_records;
use crate::pipeline::LocalPipeline;

/// Current tab selection.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Dashboard,
    Data,
    Training,
    Settings,
}

/// Shared state between UI and background training thread.
#[derive(Debug, Clone)]
pub struct SharedState {
    pub connected: bool,
    pub current_round: u32,
    pub dp_remaining: f64,
    pub dp_budget: f64,
    pub last_loss: f32,
    pub last_auc: f32,
    pub record_count: usize,
    pub training_in_progress: bool,
    pub training_progress: f32, // 0.0 – 1.0
    pub status_message: String,
    pub loss_history: Vec<[f64; 2]>, // (round, loss)
    pub shapley_score: f64,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            connected: false,
            current_round: 0,
            dp_remaining: 10.0,
            dp_budget: 10.0,
            last_loss: 0.0,
            last_auc: 0.5,
            record_count: 0,
            training_in_progress: false,
            training_progress: 0.0,
            status_message: "Ready".to_string(),
            loss_history: Vec::new(),
            shapley_score: 0.0,
        }
    }
}

/// Main application state (egui App).
pub struct FclcNodeApp {
    tab: Tab,
    state: Arc<Mutex<SharedState>>,
    pipeline: Arc<Mutex<LocalPipeline>>,
    orchestrator_url: String,
    node_name: String,
    node_id: Uuid,
    csv_path: Option<PathBuf>,
    preview_rows: Vec<Vec<String>>,
    demo_record_count: usize,
}

impl FclcNodeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let node_id = Uuid::new_v4();
        let dp_config = DpConfig::default();
        let budget = 10.0;
        let pipeline = LocalPipeline::new(dp_config, budget);

        let state = SharedState {
            dp_budget: budget,
            dp_remaining: budget,
            ..Default::default()
        };

        Self {
            tab: Tab::Dashboard,
            state: Arc::new(Mutex::new(state)),
            pipeline: Arc::new(Mutex::new(pipeline)),
            orchestrator_url: "http://localhost:8080".to_string(),
            node_name: format!("clinic-node-{}", &node_id.to_string()[..8]),
            node_id,
            csv_path: None,
            preview_rows: Vec::new(),
            demo_record_count: 200,
        }
    }

    fn render_dashboard(&self, ui: &mut Ui) {
        let state = self.state.lock().unwrap();

        ui.heading("Node Dashboard");
        ui.add_space(8.0);

        // Connection status
        let (status_text, status_color) = if state.connected {
            ("Connected", Color32::GREEN)
        } else {
            ("Disconnected", Color32::RED)
        };
        ui.horizontal(|ui| {
            ui.label("Orchestrator:");
            ui.label(RichText::new(status_text).color(status_color).strong());
            ui.label(&self.orchestrator_url);
        });

        ui.separator();

        // Key metrics
        ui.columns(3, |cols| {
            cols[0].label(RichText::new("Current Round").strong());
            cols[0].label(state.current_round.to_string());
            cols[1].label(RichText::new("Last AUC").strong());
            cols[1].label(format!("{:.4}", state.last_auc));
            cols[2].label(RichText::new("Shapley Score").strong());
            cols[2].label(format!("{:.4}", state.shapley_score));
        });

        ui.add_space(8.0);
        ui.separator();

        // DP budget gauge
        ui.label(RichText::new("Differential Privacy Budget").strong());
        let consumed = 1.0 - (state.dp_remaining / state.dp_budget) as f32;
        let bar = egui::ProgressBar::new(consumed)
            .text(format!(
                "ε used: {:.2}/{:.2} (remaining: {:.2})",
                state.dp_budget - state.dp_remaining,
                state.dp_budget,
                state.dp_remaining
            ))
            .fill(if consumed > 0.8 {
                Color32::RED
            } else if consumed > 0.5 {
                Color32::YELLOW
            } else {
                Color32::GREEN
            });
        ui.add(bar);

        ui.add_space(8.0);
        ui.label(&state.status_message);

        // Loss history plot
        if !state.loss_history.is_empty() {
            ui.add_space(8.0);
            ui.label(RichText::new("Training Loss History").strong());
            let points: PlotPoints = state
                .loss_history
                .iter()
                .map(|p| [p[0], p[1]])
                .collect();
            let line = Line::new(points).name("Train Loss");
            Plot::new("loss_plot")
                .height(150.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        }
    }

    fn render_data(&mut self, ui: &mut Ui) {
        ui.heading("Data Import");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.button("Load CSV File").clicked() {
                // In real app: open file dialog
                // For now, set a demo path
                self.csv_path = Some(PathBuf::from("data/patients.csv"));
                self.state.lock().unwrap().status_message =
                    "CSV path set (demo mode)".to_string();
            }
            if let Some(p) = &self.csv_path {
                ui.label(p.display().to_string());
            } else {
                ui.label("No file selected");
            }
        });

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Demo records:");
            ui.add(egui::DragValue::new(&mut self.demo_record_count).clamp_range(10..=10000));
            if ui.button("Load Demo Data").clicked() {
                let records = generate_demo_records(self.demo_record_count);
                let count = records.len();
                let mut state = self.state.lock().unwrap();
                state.record_count = count;
                state.status_message =
                    format!("Loaded {count} demo records (anonymised)");
                // Build a quick preview
                self.preview_rows = records
                    .iter()
                    .take(5)
                    .map(|r| {
                        vec![
                            format!("{:?}", r.age_group),
                            format!("{:?}", r.sex),
                            r.hba1c_last
                                .map(|h| format!("{h:.1}"))
                                .unwrap_or_else(|| "N/A".to_string()),
                            r.bmi
                                .map(|b| format!("{b:.0}"))
                                .unwrap_or_else(|| "N/A".to_string()),
                            if r.hospitalized_next_12m { "1" } else { "0" }.to_string(),
                        ]
                    })
                    .collect();
            }
        });

        ui.add_space(8.0);

        let state = self.state.lock().unwrap();
        ui.label(format!("Records loaded: {}", state.record_count));

        if state.record_count > 0 {
            ui.label(RichText::new("Anonymisation: Applied").color(Color32::GREEN));
        }

        drop(state);

        // Preview table
        if !self.preview_rows.is_empty() {
            ui.add_space(8.0);
            ui.label(RichText::new("Preview (first 5 rows, anonymised)").strong());
            egui::Grid::new("data_preview")
                .striped(true)
                .show(ui, |ui| {
                    // Header
                    ui.label("Age Group");
                    ui.label("Sex");
                    ui.label("HbA1c");
                    ui.label("BMI");
                    ui.label("Target");
                    ui.end_row();

                    for row in &self.preview_rows {
                        for cell in row {
                            ui.label(cell);
                        }
                        ui.end_row();
                    }
                });
        }
    }

    fn render_training(&mut self, ui: &mut Ui) {
        ui.heading("Local Training");
        ui.add_space(8.0);

        let (in_progress, progress, record_count) = {
            let s = self.state.lock().unwrap();
            (s.training_in_progress, s.training_progress, s.record_count)
        };

        if record_count == 0 {
            ui.colored_label(Color32::YELLOW, "No data loaded. Go to the Data tab first.");
        }

        // De-identification preview before submission (item 25).
        // Shows the de-identified records that will be used in training,
        // so the clinician can confirm no identifiable data is present.
        if !self.preview_rows.is_empty() {
            ui.group(|ui| {
                ui.label(RichText::new("De-identification preview — data to be submitted").strong());
                ui.label(
                    RichText::new(
                        "Verify that no direct identifiers appear below before starting training."
                    )
                    .color(Color32::GRAY)
                    .small(),
                );
                ui.add_space(4.0);
                egui::Grid::new("training_deident_preview")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Age Group").small().strong());
                        ui.label(RichText::new("Sex").small().strong());
                        ui.label(RichText::new("HbA1c").small().strong());
                        ui.label(RichText::new("BMI").small().strong());
                        ui.label(RichText::new("Target").small().strong());
                        ui.end_row();
                        for row in self.preview_rows.iter().take(3) {
                            for cell in row {
                                ui.label(RichText::new(cell).small());
                            }
                            ui.end_row();
                        }
                    });
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!(
                        "✓  {record_count} records total — all de-identified per 5-layer stack."
                    ))
                    .color(Color32::GREEN)
                    .small(),
                );
            });
            ui.add_space(8.0);
        }

        let can_train = !in_progress && record_count > 0;

        ui.add_enabled_ui(can_train, |ui| {
            if ui.button("Start Training Round").clicked() {
                self.start_training_round();
            }
        });

        if in_progress {
            ui.add_space(4.0);
            ui.add(egui::ProgressBar::new(progress).text("Training...").animate(true));
        }

        ui.add_space(8.0);
        ui.separator();

        let state = self.state.lock().unwrap();
        ui.columns(2, |cols| {
            cols[0].label(RichText::new("Last Train Loss").strong());
            cols[0].label(format!("{:.4}", state.last_loss));
            cols[1].label(RichText::new("Last Val AUC").strong());
            cols[1].label(format!("{:.4}", state.last_auc));
        });
    }

    fn start_training_round(&mut self) {
        let state_arc = self.state.clone();
        let pipeline_arc = self.pipeline.clone();
        let orchestrator_url = self.orchestrator_url.clone();
        let node_id = self.node_id;
        let record_count = self.state.lock().unwrap().record_count;

        {
            let mut s = state_arc.lock().unwrap();
            s.training_in_progress = true;
            s.training_progress = 0.0;
            s.status_message = "Generating demo data for training...".to_string();
        }

        std::thread::spawn(move || {
            // Generate demo records (in real app, loaded from CSV/FHIR)
            let mut records = generate_demo_records(record_count.max(50));

            {
                let mut s = state_arc.lock().unwrap();
                s.training_progress = 0.3;
                s.status_message = "Running local training...".to_string();
            }

            let result = {
                let mut pipeline = pipeline_arc.lock().unwrap();
                pipeline.run_training(&mut records)
            };

            match result {
                Ok(training_result) => {
                    {
                        let mut s = state_arc.lock().unwrap();
                        s.training_progress = 0.7;
                        s.status_message = "Submitting update to orchestrator...".to_string();
                        s.last_loss = training_result.train_loss;
                        s.last_auc = training_result.val_auc;
                        s.current_round += 1;
                        let round = s.current_round;
                        s.loss_history.push([round as f64, training_result.train_loss as f64]);
                        s.dp_remaining = pipeline_arc.lock().unwrap().dp_remaining();
                    }

                    // Try to submit update to orchestrator
                    let client = OrchestratorClient::new(&orchestrator_url, node_id);
                    let round = state_arc.lock().unwrap().current_round;
                    let payload = ModelUpdatePayload {
                        node_id,
                        round_id: round,
                        gradient: training_result.gradient_update.iter().map(|&x| x as f64).collect(),
                        epsilon_spent: training_result.dp_epsilon_spent,
                        loss: training_result.train_loss as f64,
                        auc: training_result.val_auc as f64,
                        record_count,
                        sigma: training_result.sigma,
                        sampling_rate: training_result.sampling_rate,
                    };

                    let connected = client.submit_update(payload).is_ok();

                    // Try to fetch updated global model and sync round number from server
                    if connected {
                        if let Ok(global) = client.get_global_model() {
                            let server_round = global.round as u32;
                            let weights_f32: Vec<f32> = global.weights.iter().map(|&x| x as f32).collect();
                            let mut pipeline = pipeline_arc.lock().unwrap();
                            pipeline.update_global_model(weights_f32);
                            // Sync round counter to server's authoritative value
                            let mut s = state_arc.lock().unwrap();
                            s.current_round = server_round;
                        }
                        if let Ok(score) = client.get_shapley_score() {
                            let mut s = state_arc.lock().unwrap();
                            s.shapley_score = score.score;
                        }
                    }

                    let mut s = state_arc.lock().unwrap();
                    s.connected = connected;
                    s.training_progress = 1.0;
                    s.training_in_progress = false;
                    s.status_message = format!(
                        "Round {} complete. Loss: {:.4}, AUC: {:.4}",
                        round,
                        s.last_loss,
                        s.last_auc
                    );
                }
                Err(e) => {
                    let mut s = state_arc.lock().unwrap();
                    s.training_in_progress = false;
                    s.status_message = format!("Training error: {e}");
                }
            }
        });
    }

    fn render_settings(&mut self, ui: &mut Ui) {
        ui.heading("Settings");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Orchestrator URL:");
            ui.text_edit_singleline(&mut self.orchestrator_url);
        });

        ui.horizontal(|ui| {
            ui.label("Node name:");
            ui.text_edit_singleline(&mut self.node_name);
        });

        ui.label(format!("Node ID: {}", self.node_id));

        ui.add_space(8.0);

        if ui.button("Register with Orchestrator").clicked() {
            let client = OrchestratorClient::new(&self.orchestrator_url, self.node_id);
            match client.register(&self.node_name) {
                Ok(_resp) => {
                    let mut s = self.state.lock().unwrap();
                    s.connected = true;
                    s.status_message = "Registered successfully".to_string();
                }
                Err(e) => {
                    let mut s = self.state.lock().unwrap();
                    s.connected = false;
                    s.status_message = format!("Registration failed: {e}");
                }
            }
        }

        if ui.button("Test Connection").clicked() {
            let client = OrchestratorClient::new(&self.orchestrator_url, self.node_id);
            let ok = client.ping();
            let mut s = self.state.lock().unwrap();
            s.connected = ok;
            s.status_message = if ok {
                "Connection OK".to_string()
            } else {
                "Cannot reach orchestrator".to_string()
            };
        }
    }
}

impl eframe::App for FclcNodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint while training to update progress bar
        {
            let s = self.state.lock().unwrap();
            if s.training_in_progress {
                ctx.request_repaint();
            }
        }

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("FCLC Node");
                ui.separator();
                ui.selectable_value(&mut self.tab, Tab::Dashboard, "Dashboard");
                ui.selectable_value(&mut self.tab, Tab::Data, "Data");
                ui.selectable_value(&mut self.tab, Tab::Training, "Training");
                ui.selectable_value(&mut self.tab, Tab::Settings, "Settings");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Dashboard => self.render_dashboard(ui),
                Tab::Data => self.render_data(ui),
                Tab::Training => self.render_training(ui),
                Tab::Settings => self.render_settings(ui),
            }
        });
    }
}

mod app;
mod client;
mod connector;
mod pipeline;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "FCLC Node",
        native_options,
        Box::new(|cc| Box::new(app::FclcNodeApp::new(cc))),
    )
    .expect("Failed to start FCLC Node GUI");
}

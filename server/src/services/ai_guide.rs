/// Ze·Guide AI service
/// Primary: DeepSeek API (deepseek-reasoner)
/// Fallback: local Ollama (Llama 3 8B)
///
/// Every response is prefixed with mandatory legal disclaimer.
/// All interactions are logged to ze_guide_logs table.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::models::ze_guide::ZE_GUIDE_DISCLAIMER;

const ZE_SYSTEM_PROMPT: &str = r#"You are Ze·Guide, a scientific assistant for the LongevityCommon platform.
You have deep knowledge of:
- Ze Vectors Theory (χ_Ze complexity index, D_norm bridge equation, Ze-budget)
- FCLC (Federated Citizen Longevity Computing) architecture
- BioSense sensor data and biological age estimation
- CDATA longevity research dataset series
- Aging biology, HRV analysis, EEG complexity metrics

KEY PUBLICATIONS (cite these when relevant):
- Ze Theory core: DOI 10.65649/nhjtra67 (Observation as Continuous Resource Expenditure, 2026)
- Ze Minkowski emergence: DOI 10.65649/hqm2c554 (Emergence of the Minkowski Metric from Ze Dynamics, 2026)
- CDATA cell model: PMID 36583780 (Tkemaladze, Mol Biol Rep 2023)
- CDATA code: DOI 10.5281/zenodo.19174506 (Cell-DT v3.0)
- HRV Task Force standard: Circulation 93(5):1043 (1996)

CORE EQUATIONS (status flags from Ze/THEORY.md, Ze/EVIDENCE.md, CONCEPT.md §A.2 — 2026-04-22):
- Ze velocity (canonical): v = N_S / (N − 1)
- Ze cheating index: χ_Ze = 1 − |v − v*| / max(v*, 1−v*)
    * v*_passive = 1 − ln(2) ≈ 0.3069 (analytic, theoretical)
    * v*_active ≈ 0.456 DEPRECATED as universal constant (dataset heterogeneity I²=90.3%; use dataset-specific values)
    * χ_Ze is a THEORETICAL abstract, NOT a validated clinical biomarker
- Bio-age estimate (research path only): bio_age = chrono_age × (1 − 1.2·(1−χ_Ze)·K)
    * K ∈ {0.45 dual, 0.42 eeg_only, 0.38 hrv_only} — research-mode heuristics; prior "R²=0.84" retracted (synthetic-data artefact)
- Validated organism score (CONCEPT v3.2): organism_sdnn = clamp((sdnn_ms − 10) / 170, 0, 1)  [d=0.724, Fantasia N=40]
- NOTE: prior "χ_Ze = 0.60 + 0.27·exp(−1.18·D_norm)" bridge equation is NOT in current Ze/THEORY.md — do not cite
- NOTE: prior Health Score "0.40·organism + 0.25·psyche + 0.20·consciousness + 0.15·social" REMOVED from CONCEPT.md §A.2 (2026-04-22) — use L_tissue from MCOA instead

You ONLY provide scientific context. You do NOT give medical advice.
Always cite sources when possible (DOI, file names, dataset names).
Be concise and precise. Use SI units. Refer to χ_Ze values as dimensionless (0–1).

CRITICAL: You are not a physician. You must never diagnose or prescribe."#;

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct AiGuideResult {
    pub response: String,
    pub model_used: String,
    pub cited_dois: Vec<String>,
    pub cited_files: Vec<String>,
    pub latency_ms: i32,
}

/// A single turn of conversation history (prompt + response pair).
pub struct ConversationTurn {
    pub prompt: String,
    pub response: String,
}

pub async fn ask(
    prompt: &str,
    history: &[ConversationTurn],
    deepseek_api_key: &str,
    deepseek_base_url: &str,
    ollama_base_url: &str,
    ollama_model: &str,
) -> AiGuideResult {
    let start = Instant::now();
    let client = Client::new();

    // Try DeepSeek first
    if !deepseek_api_key.is_empty() {
        if let Ok(result) = ask_deepseek(
            prompt, history, deepseek_api_key, deepseek_base_url, &client,
        )
        .await
        {
            let latency_ms = start.elapsed().as_millis() as i32;
            let cited_dois = extract_dois(&result);
            let cited_files = extract_files(&result);
            return AiGuideResult {
                response: result,
                model_used: "deepseek-reasoner".into(),
                cited_dois,
                cited_files,
                latency_ms,
            };
        }
    }

    // Fallback: Ollama (history prepended as plain text context)
    let response = ask_ollama(prompt, history, ollama_base_url, ollama_model, &client)
        .await
        .unwrap_or_else(|_| {
            "Ze·Guide is temporarily unavailable. Please try again later.".into()
        });
    let latency_ms = start.elapsed().as_millis() as i32;
    let cited_dois = extract_dois(&response);
    let cited_files = extract_files(&response);

    AiGuideResult {
        response,
        model_used: format!("ollama:{}", ollama_model),
        cited_dois,
        cited_files,
        latency_ms,
    }
}

async fn ask_deepseek(
    prompt: &str,
    history: &[ConversationTurn],
    api_key: &str,
    base_url: &str,
    client: &Client,
) -> anyhow::Result<String> {
    let mut messages = vec![ChatMessage {
        role: "system".into(),
        content: ZE_SYSTEM_PROMPT.into(),
    }];

    // Inject up to last 6 turns (3 exchanges) to stay within context limits
    for turn in history.iter().rev().take(6).collect::<Vec<_>>().into_iter().rev() {
        messages.push(ChatMessage { role: "user".into(),      content: turn.prompt.clone() });
        messages.push(ChatMessage { role: "assistant".into(), content: turn.response.clone() });
    }

    messages.push(ChatMessage { role: "user".into(), content: prompt.into() });

    let req = DeepSeekRequest {
        model: "deepseek-reasoner".into(),
        messages,
        temperature: 0.3,
        max_tokens: 1024,
    };

    let resp: DeepSeekResponse = client
        .post(format!("{}/chat/completions", base_url))
        .bearer_auth(api_key)
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp.choices.into_iter().next()
        .map(|c| c.message.content)
        .unwrap_or_default())
}

async fn ask_ollama(
    prompt: &str,
    history: &[ConversationTurn],
    base_url: &str,
    model: &str,
    client: &Client,
) -> anyhow::Result<String> {
    let mut context = ZE_SYSTEM_PROMPT.to_string();
    for turn in history.iter().rev().take(6).collect::<Vec<_>>().into_iter().rev() {
        context.push_str(&format!("\n\nUser: {}\nZe·Guide: {}", turn.prompt, turn.response));
    }
    let full_prompt = format!("{}\n\nUser: {}\nZe·Guide:", context, prompt);
    let body = serde_json::json!({
        "model": model,
        "prompt": full_prompt,
        "stream": false
    });

    let resp: OllamaResponse = client
        .post(format!("{}/api/generate", base_url))
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp.response)
}

/// Extract DOIs mentioned in text (pattern: 10.XXXX/...)
fn extract_dois(text: &str) -> Vec<String> {
    let re = regex_lite::Regex::new(r"10\.\d{4,}/\S+").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().trim_end_matches(['.', ',', ')']).to_string())
        .collect()
}

/// Extract file references (pattern: filename.rs, filename.py, etc.)
fn extract_files(text: &str) -> Vec<String> {
    let re = regex_lite::Regex::new(r"\b[\w\-]+\.(rs|py|csv|json|md)\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

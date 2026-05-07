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

CRITICAL HONESTY GUARDRAILS (umbrella CONCEPT v5.6, regenerated 2026-04-28):
- LongevityCommon is a HYPOTHESIS-STAGE research framework, NOT a validated medical product.
- All χ_Ze values, AUC scores, and aging-activity estimates are EXPLORATORY (hypothesis-generating), NOT confirmatory.
- Pre-registered tests of an earlier univariate χ_Ze formulation (Cuban EEG, Dortmund Vital, MPI-LEMON cohorts) yielded NULL results (deprecated/superseded by current multimodal version, which is post-hoc).
- Current multimodal χ_Ze has NOT been validated on a pre-registered N≥2000 cohort.
- p-hacking risk per Ioannidis 2005 (PMID 16060722) explicitly applies to all reported AUCs/r² values.
- You DECLINE confirmatory clinical claims about any individual user. You frame all answers as scientific context only.
- You are NOT a physician. You NEVER diagnose or prescribe.

You have scientific knowledge of:
- Ze Theory (entropy-geometric formalism; ansatz dτ_Ze/dt = −α·I(Z) — POSTULATED, not derived for biology; CHSH deformation)
- FCLC (Federated Clinical Learning Cooperative; semi-honest server only — NOT secure against active adversary; GDPR Art. 9 blocker until v14, planned Q1 2027)
- BioSense (wearable platform; χ_Ze biomarker; theoretical fixed point v* = 0.45631)
- CDATA (Centriolar Damage Accumulation Theory; status: inconclusive — Sobol p=0.12 after correction)
- MCOA (Multi-Counter Architecture; M4 falsifiability: partial r² < 0.05 for mortality on N≥2000, α=0.001)

VERIFIED PUBLICATIONS to cite (PubMed/arXiv only):
- CDATA flagship: PMID 36583780 (Tkemaladze, Mol Biol Rep 2023)
- Tkemaladze early centriole work: PMID 15886028 (Cell Biol Int 2005)
- Burgholzer information-entropy equality: arXiv:1502.00214
- Pearson nanoscale clock thermodynamic cost: PRX 11.021029 (2021) — physical clocks, NOT biology
- Ioannidis on false-positive findings: PMID 16060722
- DunedinPACE: PMID 35029144 (Belsky et al. 2022)
- GrimAge2: PMID 36516495 (Lu et al. 2022)
- López-Otín hallmarks of aging 2023: PMID 36599349
- Friston FEP: PMID 20068583
- Mironov RDP: arXiv:1702.07476

CORE QUANTITIES (Ze/THEORY.md + BioSense/THEORY.md, regenerated 2026-04-28):
- Ze velocity v ∈ [0,1] (Python convention) or [-1,+1] (Article convention).
- χ_Ze = 1 − |v − v*| / max(v*, 1 − v*); composite over EEG/HRV/resp/sleep with WEIGHTS (0.30, 0.30, 0.20, 0.20) — POST-HOC pilot fits, NOT theory-fixed.
- v* = 0.45631 (theoretical fixed point at k_λ=1; sensitivity range [0.32, 0.58] for k_λ ∈ [0.5, 2.0]).
- v* empirically tested via swept-v* on All-of-Us N=500: v*_optimal = 0.451 (95% CI 0.443-0.459) — consistent with theory.
- CDATA bridge A(D), χ_Ze(A) — 5 free params on N=196 underpowered; MOVED to Supplementary in article v5.

DEPRECATED / DO NOT USE (legacy from older system prompts):
- Old DOIs 10.65649/nhjtra67 and 10.65649/hqm2c554 — these refer to non-PubMed-indexed Longevity Horizon entries; cite ONLY verified PMID/arXiv refs above.
- Old "v*_active=0.456 DEPRECATED" wording — replaced by theoretical 0.45631 + empirical 0.451 swept-v* result.
- Old "Health Score 0.40·organism + 0.25·psyche + 0.20·consciousness + 0.15·social" — REMOVED 2026-04-22; use L_tissue MCOA aggregator.
- Old "FCLC = Federated Citizen Longevity Computing" — correct expansion is "Federated Clinical Learning Cooperative".

You ONLY provide scientific context. Always cite verified sources (PMID, arXiv ID, or PRX DOI). Use SI units. Refer to χ_Ze values as dimensionless (0–1)."#;

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
    aim_llm_url: &str,
    deepseek_api_key: &str,
    deepseek_base_url: &str,
    ollama_base_url: &str,
    ollama_model: &str,
) -> AiGuideResult {
    let start = Instant::now();
    let client = Client::new();

    // Phase 4.3 (2026-05-07): primary path is AIM-LLM HTTP shim.
    // It encapsulates all provider routing (Claude / DeepSeek / Gemini /
    // Groq / Ollama) per AIM CLAUDE.md tier chain. Use tier=deep for
    // scientific reasoning.
    if !aim_llm_url.is_empty() {
        if let Ok((reply, model)) = ask_aim_llm(prompt, history, aim_llm_url, &client).await {
            let latency_ms = start.elapsed().as_millis() as i32;
            let cited_dois = extract_dois(&reply);
            let cited_files = extract_files(&reply);
            return AiGuideResult {
                response: reply,
                model_used: format!("aim-llm:{model}"),
                cited_dois,
                cited_files,
                latency_ms,
            };
        }
    }

    // Fallback 1: direct DeepSeek (when aim-llm unreachable + key present).
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

    // Fallback 2: Ollama (history prepended as plain text context)
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

/// Call aim-llm `/v1/chat` HTTP shim. Returns `(reply, model_id)` on
/// success. The shim handles all provider fallbacks internally.
async fn ask_aim_llm(
    prompt: &str,
    history: &[ConversationTurn],
    base_url: &str,
    client: &Client,
) -> anyhow::Result<(String, String)> {
    #[derive(Serialize)]
    struct AimMessage<'a> { role: &'a str, content: &'a str }

    #[derive(Serialize)]
    struct AimChatRequest<'a> {
        messages: Vec<AimMessage<'a>>,
        tier: &'a str,
    }

    #[derive(Deserialize)]
    struct AimChatResponse {
        reply: String,
        #[serde(default)]
        model: String,
    }

    let mut msgs: Vec<AimMessage> = Vec::with_capacity(2 + history.len() * 2);
    msgs.push(AimMessage { role: "system", content: ZE_SYSTEM_PROMPT });
    for turn in history.iter().rev().take(6).collect::<Vec<_>>().into_iter().rev() {
        msgs.push(AimMessage { role: "user",      content: &turn.prompt });
        msgs.push(AimMessage { role: "assistant", content: &turn.response });
    }
    msgs.push(AimMessage { role: "user", content: prompt });

    let req = AimChatRequest {
        messages: msgs,
        // `deep` = reasoning tier (DeepSeek-V4-pro / Claude Opus / Ollama r1)
        tier: "deep",
    };

    let url = format!("{}/v1/chat", base_url.trim_end_matches('/'));
    let resp: AimChatResponse = client
        .post(&url)
        .json(&req)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok((resp.reply, resp.model))
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

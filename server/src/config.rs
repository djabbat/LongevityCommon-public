use anyhow::Context;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub deepseek_api_key: String,
    pub deepseek_base_url: String,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub crossref_base_url: String,
    pub app_host: String,
    pub app_port: u16,
    /// Comma-separated list of allowed CORS origins.
    /// Use "*" only in development. In production: "https://longevitycommon.io"
    pub allowed_origins: Vec<String>,
    /// SendGrid API key for OTP email delivery
    pub sendgrid_api_key: String,
    /// From address for OTP emails
    pub from_email: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let jwt_secret = env("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            anyhow::bail!("JWT_SECRET must be at least 32 characters long");
        }

        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Self {
            database_url: env("DATABASE_URL")?,
            jwt_secret,
            jwt_expiry_hours: env("JWT_EXPIRY_HOURS")
                .unwrap_or("24".into())
                .parse()
                .context("JWT_EXPIRY_HOURS must be a number")?,
            deepseek_api_key: env("DEEPSEEK_API_KEY").unwrap_or_default(),
            deepseek_base_url: env("DEEPSEEK_BASE_URL")
                .unwrap_or("https://api.deepseek.com/v1".into()),
            ollama_base_url: env("OLLAMA_BASE_URL")
                .unwrap_or("http://localhost:11434".into()),
            ollama_model: env("OLLAMA_MODEL").unwrap_or("llama3:8b".into()),
            crossref_base_url: env("CROSSREF_BASE_URL")
                .unwrap_or("https://api.crossref.org".into()),
            app_host: env("APP_HOST").unwrap_or("0.0.0.0".into()),
            app_port: env("APP_PORT")
                .unwrap_or("3000".into())
                .parse()
                .context("APP_PORT must be a number")?,
            allowed_origins,
            sendgrid_api_key: env("SENDGRID_API_KEY").unwrap_or_default(),
            from_email: env("FROM_EMAIL")
                .unwrap_or("noreply@longevitycommon.io".into()),
        })
    }
}

fn env(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("Missing env var: {key}"))
}

/// DOI validation via Crossref API HEAD request.
/// Invalid DOIs trigger rank_penalty on the post.

use reqwest::Client;

pub async fn verify_doi(doi: &str, crossref_base_url: &str) -> bool {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap_or_default();

    // Try Crossref HEAD request
    let url = format!("{}/works/{}", crossref_base_url, doi);
    match client.head(&url).send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => {
            // Fallback: try doi.org redirect
            let doi_url = format!("https://doi.org/{}", doi);
            client
                .head(&doi_url)
                .send()
                .await
                .map(|r| r.status().is_success())
                .unwrap_or(false)
        }
    }
}

pub const FAKE_DOI_PENALTY: f64 = 2.0;

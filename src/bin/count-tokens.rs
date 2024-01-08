use std::env;

use gcp_auth::AuthenticationManager;
use gemini_rust::{Content, CountTokensRequest, CountTokensResponse, Part};

static MODEL_NAME: &str = "gemini-pro";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:countTokens"
    );

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let prompt = "What is the airspeed of an unladen swallow?";

    let payload = CountTokensRequest {
        contents: Content {
            role: "user".to_string(),
            parts: vec![Part::Text(prompt.to_string())],
        },
    };

    let resp = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    let response = resp.json::<CountTokensResponse>().await?;

    println!("{}", response.total_tokens);

    Ok(())
}

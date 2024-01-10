use std::env;

use gcp_auth::AuthenticationManager;
use gemini_rust::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
};

static MODEL_NAME: &str = "gemini-pro-vision";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:streamGenerateContent"
    );

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let prompt = "Describe this image";
    let image_url = "gs://cloud-samples-data/generative-ai/image/320px-Felis_catus-cat_on_snow.jpg";

    let payload = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![
                Part::Text(prompt.to_string()),
                Part::FileData {
                    mime_type: "image/jpeg".to_string(),
                    file_uri: image_url.to_string(),
                },
            ],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(2048),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: None,
    };

    let resp = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    let response = resp.json::<GenerateContentResponse>().await?;
    response.0.iter().for_each(|chunk| {
        chunk.candidates.iter().for_each(|candidate| {
            candidate.content.parts.iter().for_each(|part| {
                if let Part::Text(text) = part {
                    print!("{}", text);
                }
            });
        });
    });

    Ok(())
}

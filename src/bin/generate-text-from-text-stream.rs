use std::env;

use futures_util::StreamExt;
use gcp_auth::AuthenticationManager;
use gemini_rust::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
};
use reqwest_eventsource::EventSource;

static MODEL_NAME: &str = "gemini-pro";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    // The `?alt=sse` query parameter is used to request the server to send the response as an
    // Server Sent Event (SSE).
    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:streamGenerateContent?alt=sse"
    );

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let prompt = "Tell me about the birth of the universe as a bedtime story with 1000 words.";

    let payload = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part::Text(prompt.to_string())],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(2048),
            temperature: Some(1.0),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: None,
    };

    // Build the request with the required headers and payload.
    let request = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload);

    // Delegate the request to the EventSource.
    let mut event_source = EventSource::new(request)?;

    // Iterate over the stream of events from EventSource.
    while let Some(Ok(event)) = event_source.next().await {
        match event {
            reqwest_eventsource::Event::Message(msg) => {
                let chunk = serde_json::from_str::<GenerateContentResponse>(&msg.data)?;
                let text = chunk
                    .candidates
                    .iter()
                    .flat_map(|candidate| {
                        candidate.content.parts.iter().map(|part| match part {
                            Part::Text(text) => Some(text.clone()),
                            _ => None,
                        })
                    })
                    .flatten()
                    .collect::<String>();
                print!("{}", text);
            }
            _ => (),
        }
    }

    Ok(())
}

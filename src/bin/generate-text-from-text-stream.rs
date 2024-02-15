use std::env;

use gcp_auth::AuthenticationManager;
use gemini_rust::{Content, GenerateContentRequest, GenerationConfig, Part, ResponseStreamChunk};
use reqwest::header::{self, HeaderValue};

static MODEL_NAME: &str = "gemini-pro";
static EVENT_STREAM_HEADER: HeaderValue = HeaderValue::from_static("text/event-stream");
static DATA: &str = "data: ";

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

    let mut resp = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    // Check if the server response is an SSE stream via the Content-Type header that should be
    // "text/event-stream".
    let is_sse = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .is_some_and(|header| header == EVENT_STREAM_HEADER);

    if is_sse {
        // Buffer to store partial chunks of the SSE stream.
        let mut buffer = vec![];
        while let Ok(Some(chunk)) = resp.chunk().await {
            // Append the chunk to the buffer.
            buffer.extend_from_slice(&chunk);

            // Check if the buffer ends with the end of a chunk, i.e. "\r\n\r\n". If not, keep
            // appending chunks to the buffer.
            if !buffer.ends_with(b"\r\n\r\n") {
                continue;
            }

            // The buffer contains a full chunk. Convert it to a string and clear the buffer.
            let chunk = String::from_utf8(buffer.clone())?;
            buffer.clear();

            // Ensure the chunk starts with "data: " as per the SSE spec.
            if !chunk.starts_with(DATA) {
                continue;
            }

            // Remove the "data: " prefix from the chunk, so it can be parsed as a valid JSON.
            let chunk = chunk[DATA.len()..].to_string();

            let response = serde_json::from_str::<ResponseStreamChunk>(&chunk)?;

            // Print the text content of the response.
            let text = response
                .candidates
                .iter()
                .flat_map(|c| {
                    c.content.parts.iter().map(|p| match p {
                        Part::Text(text) => Some(text.clone()),
                        _ => None,
                    })
                })
                .flatten()
                .collect::<String>();
            print!("{}", text);
        }
    }
    Ok(())
}

// https://cloud.google.com/vertex-ai/docs/generative-ai/multimodal/function-calling

use std::{collections::HashMap, env};

use gcp_auth::AuthenticationManager;
use gemini_rust::{
    Content, FunctionDeclaration, FunctionParameters, FunctionParametersProperty, GenerateContentRequest, GenerationConfig, Part, ResponseStreamChunk, Tools
};

static MODEL_NAME: &str = "gemini-pro";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let prompt = "Which theaters in Mountain View show Barbie movie?";

    let payload = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part::Text(prompt.to_string())],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(2048),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: Some(vec![Tools {
            function_declarations: Some(vec![
                FunctionDeclaration {
                    name: "find_movies".to_string(),
                    description: "find movie titles currently playing in theaters based on any description, genre, title words, etc.".to_string(),
                    parameters: FunctionParameters {
                        r#type: "object".to_string(),
                        properties: HashMap::from([
                                ("location".to_string(),  FunctionParametersProperty {
                                    r#type: "string".to_string(),
                                    description: "The city and state, e.g. San Francisco, CA or a zip code e.g. 95616".to_string(),
                                }),
                                ("description".to_string(),  FunctionParametersProperty {
                                    r#type: "string".to_string(),
                                    description: "Any kind of description including category or genre, title words, attributes, etc.".to_string(),
                                })
                        ]),
                        required: vec!["description".to_string()],
                    },
                },
                FunctionDeclaration {
                    name: "find_theaters".to_string(),
                    description: "find theaters based on location and optionally movie title which are is currently playing in theaters".to_string(),
                    parameters: FunctionParameters {
                        r#type: "object".to_string(),
                        properties: HashMap::from([
                            ("location".to_string(), FunctionParametersProperty {
                                r#type: "string".to_string(),
                                description: "The city and state, e.g. San Francisco, CA or a zip code e.g. 95616".to_string(),
                            }),
                            ("movie".to_string(), FunctionParametersProperty {
                                r#type: "string".to_string(),
                                description: "Any movie title".to_string(),
                            })
                        ]),
                        required: vec!["location".to_string()],            
                    },
                },
                FunctionDeclaration {
                    name: "get_showtimes".to_string(),
                    description: "Find the start times for movies playing in a specific theater".to_string(),
                    parameters: FunctionParameters {
                        r#type: "object".to_string(),
                        properties: HashMap::from([
                            ("location".to_string(), FunctionParametersProperty {
                                r#type: "string".to_string(),
                                description: "The city and state, e.g. San Francisco, CA or a zip code e.g. 95616".to_string(),
                            }),
                            ("movie".to_string(), FunctionParametersProperty {
                                r#type: "string".to_string(),
                                description: "Any movie title".to_string(),
                            }),
                            ("theater".to_string(), FunctionParametersProperty {
                                r#type: "string".to_string(),
                                description: "Name of the theater".to_string(),
                            }),
                            ("date".to_string(), FunctionParametersProperty {
                                r#type: "string".to_string(),
                                description: "Date for requested showtime".to_string(),
                            }),
                        ]),
                        required: vec!["location".to_string(), "movie".to_string(), "theater".to_string(), "date".to_string()],            
                    },                    
                },
            ]),
        }]),
    };

    let resp = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    let response = resp.json::<ResponseStreamChunk>().await?;

    response.candidates.iter().for_each(|candidate| {
        candidate.content.parts.iter().for_each(|part| {
            if let Part::FunctionCall {name, args} = part {
                print!("name: {}; args: {:?}", name, args);
            }
        });
    });

    Ok(())
}

# Vertex API / Gemini in Rust [![Rust](https://github.com/andreban/gemini-rust/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/andreban/gemini-rust/actions/workflows/rust.yml)

A port of the [Vertex API][4] Gemini [demos by Mete Atamel][1] to Rust.

## Running the demos

1. Enable the Vertex AI API in the [Cloud Console][2].
2. Install the [Google Cloud SDK][3] is insalled and configured and that you are logged in into an account with the Vertex API enabled in the Cloud Console. You can use `gcloud auth login` to log in.
3. Ensure you have the following environment variables set:
  - `API_ENDPOINT`: The endpoint of the Vertex AI API.
  - `PROJECT_ID`: The ID of the project to use.
  - `LOCATION_ID`: The ID of the location to use (also called region).

Alternatively, create `.cargo/config.toml` with the following content:
```toml
[env]
API_ENDPOINT="<api endpoint>"
PROJECT_ID="<project id>"
LOCATION_ID="<location id (also called region)"
```

4. Run one of the demos:
 - `generate-text-from-image-gcs`: Generates text from an image in Google Could Storage.
 - `generate-text-from-image-local`: Generate text from an local image.
 - `generate-text-from-text`: Generate text from text.

 Run with `cargo run --bin <demo name>`.


[1]: https://github.com/meteatamel/genai-samples/tree/main/vertexai/gemini/console/csharp/rest
[2]: https://console.cloud.google.com/apis/library/aiplatform.googleapis.com?q=vertex
[3]: https://cloud.google.com/sdk/?hl=en_US
[4]: https://cloud.google.com/vertex-ai/docs

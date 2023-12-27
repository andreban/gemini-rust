# Vertex API / Gemini in Rust

Aa port of the [Vertex API][4] Gemini [demos by Mete Atamel][1] to Rust.

## Running the demo

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

4. Then run the demo with `cargo run`.

[1]: https://github.com/meteatamel/genai-samples/tree/main/vertexai/gemini/console/csharp/rest
[2]: https://console.cloud.google.com/apis/library/aiplatform.googleapis.com?q=vertex
[3]: https://cloud.google.com/sdk/?hl=en_US
[4]: https://cloud.google.com/vertex-ai/docs

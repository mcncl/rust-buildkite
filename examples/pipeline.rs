use rust_buildkite::{client::Client, models::pipeline::CreatePipelineInput};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ping is an unauthenticated health check, so the token is optional.
    let token = std::env::var("BUILDKITE_TOKEN").ok();
    let client = Client::new(token);
    let _create_pipeline = client
        .pipelines()
        .create(
            "ORG",
            CreatePipelineInput {
                name: String::from("Rust Buildkite"),
                repository: String::from("github.com:ORG/REPO.git"),
                description: Some(String::from("Rust Buildkite is so good!")),
            },
        )
        .await?;
    Ok(())
}

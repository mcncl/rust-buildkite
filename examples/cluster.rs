use rust_buildkite::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ping is an unauthenticated health check, so the token is optional.
    let token = std::env::var("BUILDKITE_TOKEN").ok();
    let client = Client::new(token);

    let clusters = client.clusters().list("ORG").await?;
    println!("{clusters:?}");

    let specific_cluster = client.clusters().get("ORG", "UUID").await?;
    println!("{specific_cluster:?}");

    Ok(())
}

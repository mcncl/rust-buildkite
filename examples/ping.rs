use rust_buildkite::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ping is an unauthenticated health check, so the token is optional.
    let token = std::env::var("BUILDKITE_TOKEN").ok();
    let client = Client::new(token);

    let ping = client.ping().await?;
    println!("{ping:?}");
    Ok(())
}

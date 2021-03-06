use newsletter_in_rust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
// Bubble up the io::Error if we failed to bind the address
// Otherwise call .await on our Server
    run()?.await
}
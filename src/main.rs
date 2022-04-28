use newsletter_in_rust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
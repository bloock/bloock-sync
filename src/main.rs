mod cli;

#[tokio::main]
async fn main() {
    cli::init().await;
}

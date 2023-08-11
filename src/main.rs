mod cli;
mod translator;
mod styles;

#[tokio::main]
async fn main() {
    // assuming run is in a module called cli
    cli::run().await;
}

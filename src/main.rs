pub mod cli;
pub mod translator;
pub mod styles;

/// Oi, 'ere's where the magic happens, squire!
/// Starts up that britify machine, it does.
#[tokio::main]
async fn main() {
    // You know that run thing in the cli module? Yeah, give that a whirl, mate!
    cli::run().await;
}

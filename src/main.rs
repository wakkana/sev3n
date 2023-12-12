use sev3n::server::Sev3n;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init tracing
    init_tracing().await;
    let server = Sev3n::new();
    server.serve().await
}

async fn init_tracing() {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();
}

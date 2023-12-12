use std::error::Error;

use crate::web_server;

pub struct Sev3n {
}


impl Sev3n {
    pub fn new() -> Sev3n {
        Sev3n {}
    }

    pub async fn serve(&self) -> Result<(), Box<dyn Error>> {
        tokio::spawn(async move {
                let server = web_server::Sev3nWebServer::new();
                if let Err(e) = server.serve().await {
                    tracing::error!("Error serving: {}", e);
                }
        }).await.unwrap();
        Ok(())
    }
}
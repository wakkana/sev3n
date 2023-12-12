use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    routing::get,
    Router, response::IntoResponse, http::StatusCode,
};

pub struct Sev3nWebServer {}

struct WebState {
    msg: Mutex<HashMap<u32, Vec<String>>>,
}

impl Sev3nWebServer {
    pub fn new() -> Sev3nWebServer {
        Sev3nWebServer {}
    }

    pub async fn serve(&self) -> Result<(), Box<dyn Error>> {
        let web_data = WebState {
            msg: Mutex::new(HashMap::new()),
        };

        let app = Router::new()
            .route("/:id", get(enter))
            .route("/:id/:msg", get(send))
            .with_state(Arc::new(web_data));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:33520")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
        Ok(())
    }
}

async fn enter(Path(id): Path<u32>, State(state): State<Arc<WebState>>) -> String {
    format!(
        "Hello, {}!, msg = {}",
        id,
        state
            .msg
            .lock()
            .unwrap()
            .get(&id)
            .unwrap_or(&vec![])
            .join(",")
    )
}

async fn send(Path((id, msg)): Path<(u32, String)>, State(state): State<Arc<WebState>>) -> impl IntoResponse {
    state
        .msg
        .lock()
        .unwrap()
        .entry(id)
        .or_insert(vec![])
        .push(msg);

    StatusCode::OK
}

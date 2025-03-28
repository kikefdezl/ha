use crate::config::Config;
use axum::Router;
use axum::extract::Extension;
use axum::routing::post;
use reqwest::Client;
use serde::Serialize;

pub fn router() -> Router {
    Router::new()
        .route("/sit", post(sit))
        .route("/stand", post(stand))
}

#[derive(Serialize)]
struct LinakRequest {
    command: String,
    move_to: String,
}

impl LinakRequest {
    fn sit() -> Self {
        Self {
            command: "move_to".to_string(),
            move_to: "sit".to_string(),
        }
    }
    fn stand() -> Self {
        Self {
            command: "move_to".to_string(),
            move_to: "stand".to_string(),
        }
    }
}

async fn sit(Extension(config): Extension<Config>) -> String {
    let client = Client::new();
    let url = config.linak_service_url;
    let payload = LinakRequest::sit();
    let res = client.post(url).json(&payload).send().await.unwrap();
    res.text().await.unwrap()
}

async fn stand(Extension(config): Extension<Config>) -> String {
    let client = Client::new();
    let url = config.linak_service_url;
    let payload = LinakRequest::stand();
    let res = client.post(&url).json(&payload).send().await.unwrap();
    res.text().await.unwrap()
}

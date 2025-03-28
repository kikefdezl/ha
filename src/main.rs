use crate::config::Config;
use axum::Router;
use axum::extract::Extension;
use dotenv::dotenv;

mod config;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();

    let app = Router::new()
        .nest("/linak", routes::linak::router())
        .layer(Extension(config.clone()));

    let listener = tokio::net::TcpListener::bind(&config.host_url)
        .await
        .unwrap();
    println!("Axum server started, listening at {}", &config.host_url);
    axum::serve(listener, app).await.unwrap();
}

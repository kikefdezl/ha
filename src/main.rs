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
        .layer(Extension(config));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Axum server started, listening at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

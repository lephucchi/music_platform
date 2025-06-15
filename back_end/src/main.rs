mod auth;
mod config;
mod databases;
mod dbs;
mod dtos;
mod errors;
mod handler;
mod models;
mod routes;
mod utils;

use std::{net::SocketAddr, sync::Arc};

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::Config;
use dbs::DBClients;
use dotenv::dotenv;
use routes::create_router;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClients,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let db_client = DBClients::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());

    println!(
        "{}",
        format!("🚀 Server is running on http://localhost:{}", config.port)
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
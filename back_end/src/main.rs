use axum::{
    routing::get, Extension, Router
};
use backend::dbs::initialed_db;
use tracing::info;
use tracing_subscriber;
use std::sync::Arc;

#[tokio::main]

async fn main(){
    tracing_subscriber::fmt::init();

    let pool = initialed_db("postgresql://musicplatform:4qNolyld0a2Efld4xsfslEzzU6xa8pkg@dpg-cvvigh6uk2gs73dbj7s0-a.virginia-postgres.render.com/musicplatform?sslmode=require", 5).await;
    let app: Router = Router::new().route("/" , get(|| async {"Hello Wold"})).layer(Extension(Arc::new(pool)));
    info!("connected databased");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server is runing on port 3000");
    axum::serve(listener , app).await.unwrap();
}
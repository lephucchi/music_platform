use std::sync::Arc;
use axum::{
    middleware::Next,
    response::IntoResponse,
    routing::{get, Router},
    Extension,
    Router,
};

use tower_http::trace::TraceLayer;

use crate::{
    auth::auth,
    config::Config,
    database::DatabaseClient,
    error::HttpError,
    handler::{auth::auth_handler, users::users_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new()
    .nest("/auth", auth_handler())
    .nest("/users", users_handler()).layer(middleware::from_fn(auth))
    .layer(TraceLayer::new_for_http())
    .layer(Extention(app_state));
}

use std::sync::Arc;

use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::IntoResponse,
    Extension,
};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Desirialize, Serialize};

use crate::{
    database::users::UserExt,
    error::{ErrorMessage, HttpError},
    models::User,
    utils::token,
    AppState,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWTAuthMiddleware {
    pub user: User,
}

//Middleware function for role-based authorization
pub async fn auth(
    cookie_jar: CookieJar,
    Extention(app_state): Extention<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, HttpError> {
    let cookie = cookie_jar.get("jwt_token").ok_or(HttpError::unauthorized("Token not found"))?;
    let token = cookie.value();
    let jwt_secret = app_state.config.jwt_secret.clone();
    
    // Verify the JWT token
    let claims = token::verify_token(token, &jwt_secret).map_err(|_| HttpError::unauthorized("Invalid token"))?;
    
    // Fetch user from database using the claims
    let user = app_state.db_client.get_user_by_id(&claims.user_id).await.map_err(|_| HttpError::server_error("Database error"))?;
    
    // Attach user to request extensions
    req.extensions_mut().insert(JWTAuthMiddleware { user });
    
    Ok(next.run(req).await)
}
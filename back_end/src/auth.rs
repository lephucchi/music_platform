use std::sync::Arc;

use axum::{
    http::{Request, header},
    middleware::Next,
    response::{IntoResponse, Response},
    body::Body,
};

use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    databases::users::UserExt,
    errors::{ErrorMessage, HttpError},
    models::User,
    utils::token,
    AppState,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: User,
}

// Middleware function for role-based authorization
pub async fn auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, HttpError> {
    // Lấy Extension<AppState> từ request
    let app_state = req.extensions().get::<Arc<AppState>>().cloned().ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::TokenNotProvided.to_string())
    })?;

    // Lấy CookieJar từ request
    let cookie_jar = req.extensions().get::<CookieJar>().cloned();

    // Extract access token from cookie hoặc Authorization header
    let cookies = cookie_jar
        .as_ref()
        .and_then(|jar| jar.get("token").map(|cookie| cookie.value().to_string()))
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = cookies.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::TokenNotProvided.to_string())
    })?;

    let token_details =
        match token::decode_token(token, app_state.env.jwt_secret_key.as_bytes()) {
            Ok(token_details) => token_details,
            Err(_) => {
                return Err(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()));
            }
        };

    let user_id = uuid::Uuid::parse_str(&token_details.to_string())
        .map_err(|_| {
            HttpError::unauthorized(ErrorMessage::InvalidToken.to_string())
        })?;

    // Fetch user from database
    let user = app_state.db_client.get_user(Some(user_id), None, None)
        .await
        .map_err(|_| {
            HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
        })?;

    let user = user.ok_or_else(|| {
        HttpError::unauthorized(ErrorMessage::UserNoLongerExist.to_string())
    })?;

    // Insert the authenticated user into request extensions
    req.extensions_mut().insert(JWTAuthMiddleware {
        user: user.clone(),
    });

    Ok(next.run(req).await)
}
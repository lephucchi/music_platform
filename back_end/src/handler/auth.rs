use std::sync::Arc;

use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Extention,
    json,
    Router,
};

use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{
    database::users::UserExist,
    dtos::{FilterUserDto, LoginUserDto, RegisterUserDto, Response, UserLoginResponseDto},
    errors::{ErrorMessage, HttpError},
    utils::{password, token},
    AppState,
};

pub fn auth_handler() -> Router{
    Router::new().route("'/register", post(register)).route("/login", post(login))
}

pub async fn register(
    Extention(app_state): Extention<Arc<AppState>>, Json(body): Json<RegisterUserDto>,
)-> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client.save_user(&body.username, &body.email, &hash_password).await();

    match result {
        Ok(result) => {
            Ok((StatusCode::CREATED, Json(Response{
                status: "success",
                message: "Success register, you can login right now".to_string(),
            })))
        },

        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                
                let constraint = db_err.constraint().unwrap_or_default();

                if constraint.contains("email"){
                    Err(HttpError::unique_constrain_violation(
                        ErrorMessage::EmailExist.to_string(),
                    ))
                }
                else if constraint.contains("username"){
                    Err(HttpError::unique_constrain_violation(
                        ErrorMessage::UsernameExist.to_string(),
                    ))
                }
                else {
                    Err(HttpError::server_error("unique contains violation".to_string()))
                }
            }
            else{
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string()))
    }
}
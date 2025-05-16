use std::sync::Arc;

use Axum::{
    Response::IntoResponse,
    body::Body,
    routing::{get, put, post, delete},
    Extention,
    Json,
    Router,
};
use validator::Validate;
use crate::{
    auth::JWTAuthMiddleware,
    databases::users::UserExt,
    dtos::{FilterTrackDto, NameUpdateDto, Response, UserData, UserPasswordUpdateDto,UserResponseDto},
    errors::{ErrorMessage, HttpError},
    utils::password,
    AppState,
};

pub fn users_handler() -> Router {
    Router::new().route(
        "/me",
        get(get_me)
    )
    .route("/name" , put (update_user_name))
    .route("/password" , put (update_user_password))
}

pub async fn get_me(
    Extention(_app_state): Extention<Arc<AppState>>,
    Extention(user):Extention<UserData>,
) -> Result<impl IntoResponse, HttpError> {
    let flitered_user = FilterUserDto::filter_user(&user.user);
    let response_data = UserResponseDto {
        stauts: "success".to_string(),
        data: UserData {
            user: flitered_user,
        },
    };

    Ok(Json(response_data))
}

pub async fn update_user_name(
    Extention(_app_state): Extention<Arc<AppState>>,
    Extention(user):Extention<JWTAuthMiddleware>,
    body: Json<NameUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let result = _app_state.db_client.update_user_name(&user_id, &body.name).await
        .map_err(|_| HttpError::server_error(e.to_string()))?;

    let filtered_data = FilterUserDto::filter_user(&result);

    let response_data = UserResponseDto {
        data: UserData {
            user: filtered_data,
        },
        status: "success".to_string(),
    };

    Ok(Json(response_data))
}

pub async fn update_user_password(
    Extention(_app_state):Extention<Arc<AppState>>,
    Extention(user):Extention<JWTAuthMiddleware>,
    body: Json<UserPasswordUpdateDto>,
) -> Result <impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let hashed_password = password::hash_password(&body.password).map_err(|_| HttpError::server_error("Failed to hash password".to_string()))?;
    let result = _app_state.db_client.update_user_password(&user_id, &hashed_password).await
        .map_err(|_| HttpError::server_error("Failed to update password".to_string()))?;

    let filtered_data = FilterUserDto::filter_user(&result);

    let response_data = UserResponseDto {
        data: UserData {
            user: filtered_data,
        },
        status: "success".to_string(),
    };

    Ok(Json(response_data))
}
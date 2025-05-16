use std::sync::Arc;

use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Extension,
    json,
    Router,
};

use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{
    databases::users::UserExt,
    dtos::{FilterUserDto, LoginUserDto, RegisterUserDto, Response, UserLoginResponseDto},
    errors::{ErrorMessage, HttpError},
    utils::{password, token},
    AppState,
};

pub fn auth_handler() -> Router{
    Router::new().route("'/register", post(register)).route("/login", post(login))
}

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>, json(body): json<RegisterUserDto>,
)-> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client.save_user(&body.username, &body.email, &hash_password).await;

    match result {
        Ok(result) => {
            Ok((StatusCode::CREATED, json(Response{
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

pub fn login(
    Extention(app_state): Extention<Arc<AppState>>,
    Json(body):Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    // Kiem tra input
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))? ;

    //Fetch user
    let mut result = app_state.db_client.get_user(None, None, Some(&body.identifier)).await.map_err(|e| HttpError::server_error(e.to_string()))?;

    if result.is_none(){
        result = app_state.db_client
        .get_user(None, Some(&body.identifier), None)
        .await  
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    }

    let user = result.ok_or(HttpError::bad_request(ErrorMessage::WrongCrendentials.to_string()))?;

    //compare password
    let password_matches = password::compare(&body.password, &user.password_hash).map_err(|_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;

    if password_matches {
        //Create a JWT token
        let token = token::create_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        ).map_err(|e| HttpError::server_error(e.to_string))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage*60);
        let cookie = Cookie::build(("token" , token.clone())).path("/").max_age(cookie_duration).http_only(true).build();

        let filter_user = FilterUserDto::filter_user(&user);
        //prepare response
        let response = axum::response::Json(UserLoginResponseDto{
            status : "success".to_string(),
            user: filter_user,
            token,
        });
        let mut headers = HeaderMap::new();
        headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());
        
        let mut response = response.into_response();
        response.headers_mut().extend(headers);

        Ok(response);
    }
    else {
        Err(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))
    }
}
use chrono::NaiveDateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::{Duration, User};

#[derive(Validate, Debug, Default, Clone, Deserialize, Serialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    #[validate(custom = "validate_username")]
    pub username: String,

    #[validate(
        length(min = 1 , message = "Email must be filled"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(
        length(min = 6, message = "Password must be at least 6 characters long"),
        length(max = 12, message = "Password must be at most 12 characters long")
    )]
    pub password: String,

    #[validate(
        length(min = 1, message = "Confirm password is required"),
        must_match(other = "password" , message = "Confirm password does not match")
    )]
    pub confirm_password: String,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(username) {
        return Err(ValidationError::new("invalid_username"));
    }
    Ok(())
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(custom = "validate_identifier")]
    pub identifier: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(max = 12, message = "Password must be at most 12 characters long")
    )]
    pub password: String,
}

fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
    let email_regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]{3,}$").unwrap();

    if !email_regex.is_match(identifier) && !username_regex.is_match(identifier) {
        return Err(ValidationError::new("invalid_identifier"))
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub username: String,
    pub email: String,

    #[serde(rename = "createAt")]
    pub created_at: NaiveDateTime,

    #[serde(rename = "updateAt")]
    pub updated_at: NaiveDateTime,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize,sqlx::FromRow)]
pub struct TrackDto {
    pub id: uuid::Uuid,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration: Duration,
    pub file_name: Option<String>,
    pub upload_status: Option<String>,
    pub thumbnail_name: Option<String>,
    pub is_favorite: Option<bool>,
    pub duration_played: Duration,
    pub played_at: Option<chrono::NaiveDateTime>,
    pub is_created_by_user: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub user: FilterUserDto,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone, Default)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required"),
        length(max = 12, message = "Password must be at most 12 characters long")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "Confirm new password is required"),
        length(max = 12, message = "Password must be at most 12 characters long"),
        must_match(other = "new_password", message = "New passwords do not match"),
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required"),
        length(max = 12, message = "Password must be at most 12 characters long")
    )]
    pub old_password: String,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub track_id: uuid::Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InCompleteTrackInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub thumbnail_name: Option<String>,
    pub file_name: Option<String>,
    pub track_id: Option<uuid::Uuid>,
    pub total_chunks: i32,
    pub uploaded_chunks: i32,
    pub current_chunk: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InCompleteTractInfoResponse {
    pub incomplete_track_info: Vec<InCompleteTrackInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterTrackDto {
    pub id: uuid::Uuid,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration_minutes: f64,
    pub duration_seconds: f64,
    pub duration_played: f64,
    pub file_name: Option<String>,
    pub thumbnail_name: Option<String>,
    pub is_favorite: Option<bool>,
    pub played_at: Option<chrono::NaiveDateTime>,
    pub is_created_by_user: Option<bool>,
}

impl FilterTrackDto {
    pub fn filter_track(track: &TrackDto) -> Self {
        FilterTrackDto {
            id: track.id,
            title: track.title.clone(),
            artist: track.artist.clone(),
            duration_minutes: convert_duration_to_minutes(&track.duration),
            duration_seconds: convert_duration_to_seconds(&track.duration),
            duration_played: convert_duration_to_seconds(&track.duration_played),
            file_name: track.file_name.clone(),
            thumbnail_name: track.thumbnail_name.clone(),
            is_favorite: track.is_favorite,
            played_at: track.played_at,
            is_created_by_user: track.is_created_by_user,
        }
    }

    pub fn filter_tracks(tracks: &[TrackDto]) -> Vec<FilterTrackDto> {
        tracks.iter().map(Self::filter_track).collect()
    }
}

fn convert_duration_to_minutes(duration: &Duration) -> f64 {
    let total_seconds = (duration.months * 30 * 24 * 60 * 60) as f64
        + (duration.days * 24 * 60 * 60) as f64
        + (duration.microseconds as f64 / 1_000_000.0);
    total_seconds / 60.0
}

fn convert_duration_to_seconds(duration: &Duration) -> f64 {
    (duration.months * 30 * 24 * 60 * 60) as f64
        + (duration.days * 24 * 60 * 60) as f64
        + (duration.microseconds as f64 / 1_000_000.0)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackResponseDto {
    pub tracks: Vec<FilterTrackDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFavoritesDto {
    pub track_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayListDto {
    pub id: uuid::Uuid,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub max_track_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayListResponse {
    pub playlists: Vec<PlayListDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTrackPlayList {
    pub playlist_id: uuid::Uuid,
    pub track_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaybackMessageDto {
    pub track_id: uuid::Uuid,
    pub duration_played: i64,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}
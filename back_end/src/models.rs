use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use sqlx::FromRow;
use sqlx::postgres::types::PgInterval;


#[derive(Debug, Serialize , Deserialize , Clone)]
pub struct Duration {
    pub months: i32,
    pub days: i32,
    pub microseconds: i32,
}

impl Duration {
    fn default() -> Self {
        Self {
            months: 0,
            days: 0,
            microseconds: 0,
        }
    }
}

impl From<PgInterval> for Duration {
    fn from(interval: PgInterval) -> Self {
        Self {
            months: interval.months,
            days: interval.days,
            microseconds: interval.microseconds,
        }
    }
}

impl From<Option<PgInterval>> for Duration {
    fn from(option: Option<PgInterval>) -> Self {
        match option {
            Some(interval) => Duration::from(interval),
            None => Duration::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub durations: Duration,
    pub file_name: Option<String>,
    pub upload_status: Option<String>,
    pub thumbnail_name: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AudioFile {
    pub id: Uuid,
    pub track_id: Option<Uuid>,
    pub total_chunks: i32,
    pub upload_chunks: i32,
    pub current_chunk: i32,
    pub chunk_path: Option<String>,
    pub upload_status: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlaylistTrack {
    pub playlist_id: Uuid,
    pub track_id: Uuid,
    pub track_oder: i32
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlaybackHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub track_id: Uuid,
    pub played_at: NaiveDateTime,
    pub duration_played: Duration

}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserFavorite{
    pub id: Uuid,
    pub user_id: Uuid,
    pub track_id: Uuid,
    pub created_at: NaiveDateTime,
}



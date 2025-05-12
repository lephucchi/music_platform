use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{postgres::types::PgInterval, query, query_as, Error};

use crate::{dbs::DBClients, dtos::InCompleteTractInfo, models::AudioFile};

#[async_trait]
pub trait UploadExt {
    async fn upload_file(
        &self,
        user_id: Uuid,
        file_name: String,
    ) -> Result<Uuid, sqlx::Error>;

    async fn upload_chuck(
        &self,
        track_id: Uuid,
        total_chunks: i32,
        uploaded_chunks: i32,
        current_chunks: i32,
        chunk_path: &String,
    ) -> Result<(), sqlx::Error>;

    async fn get_audio_file(
        &self,
        track_id: Uuid,
    ) -> Result<Option<AudioFile>, sqlx::Error>;

    async fn upload_thumbnail(
        &self,
        track_id: Uuid,
        thumbnail_name: String,
        title :String,
        artitst: String,
    ) -> Result<(), sqlx::Error>;

    async fn upload_status(
        &self,
        track_id: Uuid,
        duration: i64,
    ) -> Result<(), sqlx::Error>;

    async fn get_incomplete_upload(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<InCompleteTractInfo>, sqlx::Error>;
}

#[async_trait]
impl UploadExt for DBClients{
    async fn upload_file(
        &self,
        user_id: Uuid,
        file_name: String,
    ) -> Result<Uuid, sqlx::Error>{
        let query = sqlx::query!(
            r#"
            INSERT INTO tracks (user_id, file_name)
            VALUES ($1, $2)
            RETURNING id
            "#,
            user_id,
            file_name
        ).fetch_one(&self.pool).await?;

        Ok(query.id)
    }

    async fn upload_chuck(
        &self,
        track_id: Uuid,
        total_chunks: i32,
        uploaded_chunks: i32,
        current_chunks: i32,
        chunk_path: &String,
    ) -> Result<(), sqlx::Error>{
        let query = sqlx::query_as!(
            AudioFile,
        )

        Ok(())
    }

    async fn get_audio_file(
        &self,
        track_id: Uuid,
    ) -> Result<Option<AudioFile>, sqlx::Error>;

    async fn upload_thumbnail(
        &self,
        track_id: Uuid,
        thumbnail_name: String,
        title :String,
        artitst: String,
    ) -> Result<(), sqlx::Error>;

    async fn upload_status(
        &self,
        track_id: Uuid,
        duration: i64,
    ) -> Result<(), sqlx::Error>;

    async fn get_incomplete_upload(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<InCompleteTractInfo>, sqlx::Error>;

}

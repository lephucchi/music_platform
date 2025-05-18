use async_trait::async_trait;
use uuid::Uuid;

use crate::{dbs::DBClients, dtos::TrackDto};

#[async_trait]
pub trait TrackExt {
    async fn get_random_tracks(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrackDto>, sqlx::Error>;
}

#[async_trait]
impl TrackExt for DBClients {
    async fn get_random_tracks(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrackDto>, sqlx::Error> {
        let tracks = sqlx::query_as!(
            TrackDto,
            r#"
            SELECT
                t.id,
                t.title,
                t.artist,
                t.duration,
                t.file_name,
                t.upload_status,
                t.thumbnail_name,
                COALESCE(ph.played_at, NULL) AS played_at,
                CASE WHEN uf.id IS NOT NULL THEN true ELSE false END AS is_favorite,
                COALESCE(ph.duration_played, INTERVAL '0 seconds') AS duration_played,
                CASE WHEN t.user_id = $1 THEN true ELSE false END AS is_created_by_user
            FROM tracks t
            LEFT JOIN user_favorites uf
                ON uf.track_id = t.id AND uf.user_id = $1
            LEFT JOIN playback_history ph
                ON ph.track_id = t.id AND ph.user_id = $1
            WHERE t.upload_status = 'complete'
            ORDER BY RANDOM()
            LIMIT 20
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tracks)
    }
}

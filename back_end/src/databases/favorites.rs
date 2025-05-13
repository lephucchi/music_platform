use async_trait::async_trait;
use uuid::Uuid;

use crate::{dbs::DBClients, dtos::TrackDto};

#[async_trait]
pub trait FavoriteExt {
    async fn  save_favorite(
        &self,
        track_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error>;

    async fn delete_favorite(
        &self,
        track_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error>;

    async fn get_user_favorite_tracks(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrackDto>, sqlx::Error>;
}

#[async_trait]
impl FavoriteExt for DBClients {
    async fn save_favorite(
        &self,
        track_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let query = r#"
            INSERT INTO favorites (track_id, user_id)
            VALUES ($1, $2)
            RETURNING id, track_id, user_id, created_at
        "#;

        let favorite = sqlx::query_as::<_, Favorite>(query)
            .bind(track_id)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_favorite(
        &self,
        track_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let query = r#"
            DELETE FROM favorites
            WHERE track_id = $1 AND user_id = $2
        "#;

        sqlx::query(query)
            .bind(track_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_user_favorite_tracks(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Favorite>, sqlx::Error> {
        let tracks = sqlx::query_as!(
            TrackDto,
            r#"
            SELECT
                t.id, t.name, t.artist, t.album, t.duration, f.file_name, t.upload_status, t.thumbnail_name,
                true AS is_favorite, 
                COALESCE(ph.played_at, NULL) AS played_at,
                COALESCE(ph.duration, INTERVAL '0 seconds') AS duration_played,
                CASE WHEN t.user_id = $1 THEN true ELSE false END AS is_created_by_user,
            FROM 
                tracks t
            Join
                user_favorites uf ON t.id = uf.track_id
            LEFT JOIN
                playback_history ph ON t.id = ph.track_id AND ph.user_id = $1
            WHERE 
                uf.user_id = $1
            AND
                t.upload_status = 'success'
            "#,
            user_id
        ).fetch_all(&self.pool).await?;

        Ok(tracks)
    }
}
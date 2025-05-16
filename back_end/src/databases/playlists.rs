use async_trait::async_trait;
use symphonia::core::formats::Track;
use uuid::Uuid;

use crate::{dbs::DBClients, dtos::{PlayListDto, TrackDto}};

#[async_trait]
pub trait PlayListsExt {
    async fn create_playlist(
        &self,
        user_id: Uuid,
        title: String,
        thumbnail_path: String,
    ) -> Result<(), sqlx::Error>;

    async fn get_last_track_order (
        &self,
        playlist_id: Uuid,
    ) -> Result<(i32), sqlx::Error>;
    
    async fn add_track_to_playlist(
        &self,
        playlist_id: Uuid,
        track_id: Uuid,
        track_order: i32,
    ) -> Result<(), sqlx::Error>;

    async fn get_user_playlists(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TrackDto>, sqlx::Error>;

    async fn get_playlist_tracks(
        &self,
        playlist_id: Uuid, 
        user_id: Uuid
    ) -> Result<Vec<TrackDto>, sqlx::Error>;
}

#[async_trait]
impl PlayListsExt for DBClients {
    async fn create_playlist(
        &self,
        user_id: Uuid,
        title: String,
        thumbnail_path: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO playlists (user_id, title, thumbnail_path)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            title,
            thumbnail_path,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_last_track_order (
        &self,
        playlist_id: Uuid,
    ) -> Result<(i32), sqlx::Error>{
        let query = sqlx::query!(
            r#"
            SELECT track_order
            FROM playlist_tracks
            WHERE playlist_id = $1
            ORDER BY track_order DESC
            "#,
            playlist_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((query.track_order.unwrap_or(0)) as i32)
    }
    
    async fn add_track_to_playlist(
        &self,
        playlist_id: Uuid,
        track_id: Uuid,
        track_order: i32,
    ) -> Result<(), sqlx::Error>{
        sqlx::query!(
            r#"
            INSERT INTO playlist_tracks (playlist_id, track_id, track_order)
            VALUES ($1, $2, $3)
            "#,
            playlist_id,
            track_id,
            track_order,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    async fn get_user_playlists(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<PlayListDto>, sqlx::Error> {
        let query = sqlx::query_as!(
            PlayListDto,
            r#"
            SELECT 
                p.id,
                p.title,
                p.thumbnail_path,
                COALESCE(MAX(pt.track_order), 0) as max_track_order
            FROM playlists p
            LEFT JOIN playlist_tracks pt ON p.id = pt.playlist_id
            WHERE p.user_id = $1
            GROUP BY p.id, p.title, p.thumbnail_path
            ORDER BY p.title
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query)
    }
    async fn get_playlist_tracks(
        &self,
        playlist_id: Uuid, 
        user_id: Uuid
    ) -> Result<Vec<TrackDto>, sqlx::Error>{
        let query = sqlx::query_as!(
            TrackDto,
            r#"
            SELECT 
                t.id, t.title, t.artist, t.duration, t.file_name, t.upload_status, t.thumbnail_name, 
                COALESCE(ph.played_at, NULL) AS played_at,
                CASE WHEN uf.track_id IS NOT NULL THEN true ELSE false END as "is_favorite?",
                COALESCE(ph.duration_played, INTERVAL '0 seconds') AS duration_played, -- Default to 0 if no playback history
                CASE WHEN t.user_id = $2 THEN true ELSE false END as is_created_by_user
            FROM playlist_tracks pt
            INNER JOIN tracks t ON pt.track_id = t.id
            LEFT JOIN user_favorites uf ON t.id = uf.track_id AND uf.user_id = $2
            LEFT JOIN playback_history ph ON t.id = ph.track_id AND ph.user_id = $2
            WHERE pt.playlist_id = $1
            "#,
            playlist_id,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(query)
    }
}
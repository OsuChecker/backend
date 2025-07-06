use sqlx::{PgPool, PgConnection, Executor};
use chrono::Utc;
use crate::models::ranked::{RankedMatch, STATUS_WAITING_PLAYER, STATUS_PLAYING, STATUS_COMPLETED};

impl RankedMatch {
    /// Crée un nouveau match ranked
    pub async fn create<'c, E>(
        executor: E,
        player1_id: i32,
        match_type: String,
        mode: i32,
        best_of: i32,
        preparation_duration: Option<i32>,
        play_duration: Option<i32>,
    ) -> Result<Self, sqlx::Error> 
    where
        E: Executor<'c, Database = sqlx::Postgres>,
    {
        let now = Utc::now().naive_utc();
        
        let match_data = sqlx::query_as!(
            RankedMatch,
            r#"
            INSERT INTO ranked_match (
                player1_id, status, match_type, mode, best_of,
                preparation_duration, play_duration,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            RETURNING *
            "#,
            player1_id,
            STATUS_WAITING_PLAYER,
            match_type,
            mode,
            best_of,
            preparation_duration.unwrap_or(30),  // 30 secondes par défaut
            play_duration.unwrap_or(300),        // 5 minutes par défaut
            now
        )
        .fetch_one(executor)
        .await?;

        Ok(match_data)
    }

    /// Récupère un match par son ID
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let match_data = sqlx::query_as!(
            RankedMatch,
            r#"
            SELECT * FROM ranked_match WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(match_data)
    }

    /// Rejoint un match en tant que deuxième joueur
    pub async fn join<'c, E>(&mut self, executor: E, player2_id: i32) -> Result<(), sqlx::Error>
    where
        E: Executor<'c, Database = sqlx::Postgres>,
    {
        let now = Utc::now().naive_utc();

        sqlx::query!(
            r#"
            UPDATE ranked_match 
            SET player2_id = $1,
                status = $2,
                updated_at = $3
            WHERE id = $4
            "#,
            player2_id,
            STATUS_PLAYING,
            now,
            self.id
        )
        .execute(executor)
        .await?;

        self.player2_id = Some(player2_id);
        self.status = STATUS_PLAYING.to_string();
        self.updated_at = now;

        Ok(())
    }

    /// Met à jour les points d'un joueur
    pub async fn add_point(&mut self, pool: &PgPool, player_id: i32) -> Result<(), sqlx::Error> {
        let now = Utc::now().naive_utc();
        
        let result = sqlx::query!(
            r#"
            UPDATE ranked_match 
            SET player1_points = CASE 
                    WHEN $1 = player1_id THEN player1_points + 1 
                    ELSE player1_points 
                END,
                player2_points = CASE 
                    WHEN $1 = player2_id THEN player2_points + 1 
                    ELSE player2_points 
                END,
                updated_at = $2
            WHERE id = $3
            RETURNING player1_points, player2_points
            "#,
            player_id,
            now,
            self.id
        )
        .fetch_one(pool)
        .await?;

        self.player1_points = result.player1_points;
        self.player2_points = result.player2_points;
        self.updated_at = now;

        // Vérifier si le match est terminé
        let max_points = (self.best_of / 2) + 1;
        if self.player1_points >= max_points || self.player2_points >= max_points {
            sqlx::query!(
                r#"
                UPDATE ranked_match 
                SET status = $1,
                    updated_at = $2
                WHERE id = $3
                "#,
                STATUS_COMPLETED,
                now,
                self.id
            )
            .execute(pool)
            .await?;

            self.status = STATUS_COMPLETED.to_string();
        }

        Ok(())
    }

    /// Récupère les matchs en cours d'un joueur
    pub async fn get_active_matches_for_player(pool: &PgPool, player_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let matches = sqlx::query_as!(
            RankedMatch,
            r#"
            SELECT * FROM ranked_match 
            WHERE (player1_id = $1 OR player2_id = $1)
            AND status != $2
            ORDER BY created_at DESC
            "#,
            player_id,
            STATUS_COMPLETED
        )
        .fetch_all(pool)
        .await?;

        Ok(matches)
    }

    /// Récupère l'historique des matchs d'un joueur
    pub async fn get_match_history_for_player(
        pool: &PgPool,
        player_id: i32,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Result<Vec<Self>, sqlx::Error> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let matches = sqlx::query_as!(
            RankedMatch,
            r#"
            SELECT * FROM ranked_match 
            WHERE (player1_id = $1 OR player2_id = $1)
            AND status = $2
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            player_id,
            STATUS_COMPLETED,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(matches)
    }
} 
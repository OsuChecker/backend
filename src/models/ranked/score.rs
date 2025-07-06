use sqlx::PgPool;
use chrono::Utc;
use crate::models::ranked::RankedMatchRoundScore;
use crate::models::score::score::Score;

impl RankedMatchRoundScore {
    /// Enregistre un nouveau score pour un round
    pub async fn create(
        pool: &PgPool,
        round_id: i32,
        player_id: i32,
        score_id: i32
    ) -> Result<Self, sqlx::Error> {
        let now = Utc::now().naive_utc();
        
        let round_score = sqlx::query_as!(
            RankedMatchRoundScore,
            r#"
            INSERT INTO ranked_match_round_scores (
                round_id, player_id, score_id, created_at
            )
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            round_id,
            player_id,
            score_id,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(round_score)
    }

    /// Récupère tous les scores d'un round
    pub async fn get_scores_for_round(pool: &PgPool, round_id: i32) -> Result<Vec<Score>, sqlx::Error> {
        let scores = sqlx::query_as!(
            Score,
            r#"
            SELECT s.* 
            FROM ranked_match_round_scores rs
            JOIN score s ON s.id = rs.score_id
            WHERE rs.round_id = $1
            ORDER BY rs.created_at ASC
            "#,
            round_id
        )
        .fetch_all(pool)
        .await?;

        Ok(scores)
    }

    /// Récupère tous les scores d'un joueur dans un round
    pub async fn get_player_scores_in_round(
        pool: &PgPool,
        round_id: i32,
        player_id: i32
    ) -> Result<Vec<Score>, sqlx::Error> {
        let scores = sqlx::query_as!(
            Score,
            r#"
            SELECT s.* 
            FROM ranked_match_round_scores rs
            JOIN score s ON s.id = rs.score_id
            WHERE rs.round_id = $1 AND rs.player_id = $2
            ORDER BY rs.created_at ASC
            "#,
            round_id,
            player_id
        )
        .fetch_all(pool)
        .await?;

        Ok(scores)
    }

    /// Récupère le meilleur score d'un joueur dans un round
    pub async fn get_player_best_score_in_round(
        pool: &PgPool,
        round_id: i32,
        player_id: i32
    ) -> Result<Option<Score>, sqlx::Error> {
        let score = sqlx::query_as!(
            Score,
            r#"
            SELECT s.* 
            FROM ranked_match_round_scores rs
            JOIN score s ON s.id = rs.score_id
            WHERE rs.round_id = $1 AND rs.player_id = $2
            ORDER BY s.score DESC
            LIMIT 1
            "#,
            round_id,
            player_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(score)
    }
} 
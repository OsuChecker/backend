use sqlx::PgPool;
use chrono::Utc;
use crate::models::ranked::{
    RankedMatchRound,
    ROUND_STATUS_PREPARING,
    ROUND_STATUS_PLAYING,
    ROUND_STATUS_COMPLETED
};
use crate::models::map::beatmap::Beatmap;

impl RankedMatchRound {
    /// Crée un nouveau round pour un match
    pub async fn create(
        pool: &PgPool,
        match_id: i32,
        round_number: i32,
        beatmap_id: i32
    ) -> Result<Self, sqlx::Error> {
        let now = Utc::now().naive_utc();
        
        let round = sqlx::query_as!(
            RankedMatchRound,
            r#"
            INSERT INTO ranked_match_round (
                match_id, round_number, beatmap_id, status,
                preparation_start, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $5, $5)
            RETURNING *
            "#,
            match_id,
            round_number,
            beatmap_id,
            ROUND_STATUS_PREPARING,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(round)
    }

    /// Récupère un round par son ID
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let round = sqlx::query_as!(
            RankedMatchRound,
            r#"
            SELECT * FROM ranked_match_round WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(round)
    }

    /// Récupère le round actuel d'un match
    pub async fn get_current_round(pool: &PgPool, match_id: i32) -> Result<Option<Self>, sqlx::Error> {
        let round = sqlx::query_as!(
            RankedMatchRound,
            r#"
            SELECT * FROM ranked_match_round 
            WHERE match_id = $1 
            AND status != $2
            ORDER BY round_number DESC 
            LIMIT 1
            "#,
            match_id,
            ROUND_STATUS_COMPLETED
        )
        .fetch_optional(pool)
        .await?;

        Ok(round)
    }

    /// Met à jour le statut "prêt" d'un joueur
    pub async fn set_player_ready(&mut self, pool: &PgPool, player_number: i32) -> Result<bool, sqlx::Error> {
        let now = Utc::now().naive_utc();

        // Met à jour le statut ready du joueur
        let result = if player_number == 1 {
            sqlx::query_as!(
                RankedMatchRound,
                r#"
                UPDATE ranked_match_round 
                SET player1_ready = true,
                    updated_at = $1
                WHERE id = $2
                RETURNING *
                "#,
                now,
                self.id
            )
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_as!(
                RankedMatchRound,
                r#"
                UPDATE ranked_match_round 
                SET player2_ready = true,
                    updated_at = $1
                WHERE id = $2
                RETURNING *
                "#,
                now,
                self.id
            )
            .fetch_one(pool)
            .await?
        };

        self.player1_ready = result.player1_ready;
        self.player2_ready = result.player2_ready;
        self.updated_at = result.updated_at;

        // Si les deux joueurs sont prêts, on passe à la phase de jeu
        if self.player1_ready && self.player2_ready && self.status == ROUND_STATUS_PREPARING {
            let result = sqlx::query_as!(
                RankedMatchRound,
                r#"
                UPDATE ranked_match_round 
                SET status = $1,
                    play_start = $2,
                    updated_at = $2
                WHERE id = $3
                RETURNING *
                "#,
                ROUND_STATUS_PLAYING,
                now,
                self.id
            )
            .fetch_one(pool)
            .await?;

            self.status = result.status;
            self.play_start = result.play_start;
            self.updated_at = result.updated_at;
            return Ok(true);
        }

        Ok(false)
    }

    /// Met à jour le meilleur score d'un joueur
    pub async fn update_best_score(
        &mut self,
        pool: &PgPool,
        player_number: i32,
        score_id: i32
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().naive_utc();
        
        let result = if player_number == 1 {
            sqlx::query_as!(
                RankedMatchRound,
                r#"
                UPDATE ranked_match_round 
                SET player1_best_score_id = $1,
                    updated_at = $2
                WHERE id = $3
                RETURNING *
                "#,
                score_id,
                now,
                self.id
            )
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_as!(
                RankedMatchRound,
                r#"
                UPDATE ranked_match_round 
                SET player2_best_score_id = $1,
                    updated_at = $2
                WHERE id = $3
                RETURNING *
                "#,
                score_id,
                now,
                self.id
            )
            .fetch_one(pool)
            .await?
        };

        self.player1_best_score_id = result.player1_best_score_id;
        self.player2_best_score_id = result.player2_best_score_id;
        self.updated_at = result.updated_at;

        Ok(())
    }

    /// Termine le round et détermine le gagnant
    pub async fn complete_round(&mut self, pool: &PgPool) -> Result<Option<i32>, sqlx::Error> {
        let now = Utc::now().naive_utc();

        // Déterminer le gagnant basé sur les meilleurs scores
        let winner_id = match (self.player1_best_score_id, self.player2_best_score_id) {
            (Some(score1_id), Some(score2_id)) => {
                let score1 = sqlx::query!(
                    "SELECT score FROM score WHERE id = $1",
                    score1_id
                )
                .fetch_one(pool)
                .await?;

                let score2 = sqlx::query!(
                    "SELECT score FROM score WHERE id = $1",
                    score2_id
                )
                .fetch_one(pool)
                .await?;

                if score1.score > score2.score {
                    Some(1)  // Player 1 wins
                } else {
                    Some(2)  // Player 2 wins
                }
            }
            (Some(_), None) => Some(1),  // Player 1 wins by default
            (None, Some(_)) => Some(2),  // Player 2 wins by default
            (None, None) => None,        // No winner
        };

        // Mettre à jour le round
        let result = sqlx::query_as!(
            RankedMatchRound,
            r#"
            UPDATE ranked_match_round 
            SET status = $1,
                winner_id = $2,
                ended_at = $3,
                updated_at = $3
            WHERE id = $4
            RETURNING *
            "#,
            ROUND_STATUS_COMPLETED,
            winner_id,
            now,
            self.id
        )
        .fetch_one(pool)
        .await?;

        self.status = result.status;
        self.winner_id = result.winner_id;
        self.ended_at = result.ended_at;
        self.updated_at = result.updated_at;

        Ok(winner_id)
    }

    /// Récupère tous les rounds d'un match
    pub async fn get_rounds_for_match(pool: &PgPool, match_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let rounds = sqlx::query_as!(
            RankedMatchRound,
            r#"
            SELECT * FROM ranked_match_round 
            WHERE match_id = $1 
            ORDER BY round_number ASC
            "#,
            match_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rounds)
    }
} 
use sqlx::PgPool;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use utoipa::ToSchema;

use super::{RankedMatch, MATCH_TYPE_FIVE_MINUTES};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RankedQueue {
    pub id: i32,
    pub user_id: i32,
    pub mode: i32,
    pub created_at: NaiveDateTime,
}

impl RankedQueue {
    /// Ajoute un joueur dans la queue
    pub async fn join_queue(pool: &PgPool, user_id: i32, mode: i32) -> Result<Self, sqlx::Error> {
        let now = Utc::now().naive_utc();

        // Supprimer d'abord toute entrée existante pour ce joueur
        sqlx::query!(
            r#"
            DELETE FROM ranked_queue WHERE user_id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        // Ajouter le joueur dans la queue
        let queue_entry = sqlx::query_as!(
            RankedQueue,
            r#"
            INSERT INTO ranked_queue (user_id, mode, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user_id,
            mode,
            now
        )
        .fetch_one(pool)
        .await?;

        // Essayer de matcher avec un autre joueur
        Self::try_match_players(pool, mode).await?;

        Ok(queue_entry)
    }

    /// Retire un joueur de la queue
    pub async fn leave_queue(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM ranked_queue WHERE user_id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Vérifie si un joueur est dans la queue
    pub async fn is_in_queue(pool: &PgPool, user_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM ranked_queue WHERE user_id = $1) as "exists!"
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.exists)
    }

    /// Essaie de matcher des joueurs dans la queue
    async fn try_match_players(pool: &PgPool, mode: i32) -> Result<Option<RankedMatch>, sqlx::Error> {
        // Commencer une transaction
        let mut tx = pool.begin().await?;

        // Trouver les deux joueurs les plus anciens dans la queue pour ce mode
        let players = sqlx::query!(
            r#"
            SELECT user_id 
            FROM ranked_queue 
            WHERE mode = $1
            ORDER BY created_at ASC 
            LIMIT 2
            "#,
            mode
        )
        .fetch_all(&mut *tx)
        .await?;

        // Si on a deux joueurs, créer un match
        if players.len() == 2 {
            let player1_id = players[0].user_id;
            let player2_id = players[1].user_id;

            // Créer le match
            let match_data = RankedMatch::create(
                &mut *tx,
                player1_id,
                MATCH_TYPE_FIVE_MINUTES.to_string(),
                mode,
                5, // Best of 5 par défaut
                None,
                None,
            ).await?;

            // Faire rejoindre le deuxième joueur
            let mut match_data = match_data;
            match_data.join(&mut *tx, player2_id).await?;

            // Supprimer les joueurs de la queue
            sqlx::query!(
                r#"
                DELETE FROM ranked_queue 
                WHERE user_id = ANY($1)
                "#,
                &[player1_id, player2_id]
            )
            .execute(&mut *tx)
            .await?;

            // Commit la transaction
            tx.commit().await?;

            Ok(Some(match_data))
        } else {
            // Pas assez de joueurs, annuler la transaction
            tx.rollback().await?;
            Ok(None)
        }
    }

    /// Récupère le nombre de joueurs en attente par mode
    pub async fn get_queue_counts(pool: &PgPool) -> Result<Vec<(i32, i64)>, sqlx::Error> {
        let counts = sqlx::query!(
            r#"
            SELECT mode, COUNT(*) as "count!"
            FROM ranked_queue
            GROUP BY mode
            ORDER BY mode
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(counts.into_iter().map(|r| (r.mode, r.count)).collect())
    }
} 
use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::{
    models::{
        ranked::{
            queue::RankedQueue,
            RankedMatch,
            RankedMatchRound,
            RankedMatchRoundScore,
        },
        user::user::User,
    },
    db::DatabaseManager,
};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct QueueStatus {
    pub in_queue: bool,
    pub queue_counts: Vec<(i32, i64)>,  // (mode, count)
}

#[derive(Debug, Deserialize)]
pub struct JoinQueueRequest {
    pub mode: i32,
}

#[derive(Debug, Serialize)]
pub struct MatchStatus {
    pub match_data: RankedMatch,
    pub current_round: Option<RankedMatchRound>,
    pub player1: User,
    pub player2: Option<User>,
}

/// Rejoint la queue ranked
pub async fn join_queue(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Json(req): Json<JoinQueueRequest>,
) -> Result<Json<QueueStatus>, (StatusCode, String)> {
    // Vérifier si le mode est valide
    if !(0..=3).contains(&req.mode) {
        return Err((StatusCode::BAD_REQUEST, "Mode invalide".to_string()));
    }

    // Vérifier si le joueur n'est pas déjà dans un match
    let active_matches = RankedMatch::get_active_matches_for_player(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !active_matches.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Vous êtes déjà dans un match".to_string()));
    }

    // Ajouter le joueur à la queue
    RankedQueue::join_queue(&pool, user.id, req.mode)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Récupérer le statut de la queue
    let in_queue = RankedQueue::is_in_queue(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let queue_counts = RankedQueue::get_queue_counts(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(QueueStatus {
        in_queue,
        queue_counts,
    }))
}

/// Quitte la queue ranked
pub async fn leave_queue(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
) -> Result<StatusCode, (StatusCode, String)> {
    RankedQueue::leave_queue(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

/// Récupère le statut de la queue
pub async fn get_queue_status(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
) -> Result<Json<QueueStatus>, (StatusCode, String)> {
    let in_queue = RankedQueue::is_in_queue(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let queue_counts = RankedQueue::get_queue_counts(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(QueueStatus {
        in_queue,
        queue_counts,
    }))
}

/// Récupère le statut du match en cours
pub async fn get_match_status(
    State(pool): State<PgPool>,
        Extension(user): Extension<User>,
) -> Result<Json<MatchStatus>, (StatusCode, String)> {
    // Récupérer le match actif du joueur
    let active_matches = RankedMatch::get_active_matches_for_player(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let match_data = active_matches.first()
        .ok_or((StatusCode::NOT_FOUND, "Aucun match en cours".to_string()))?;

    // Récupérer les informations des joueurs
    let player1 = User::get_by_id(&pool, match_data.player1_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Joueur 1 non trouvé".to_string()))?;

    let player2 = if let Some(player2_id) = match_data.player2_id {
        User::get_by_id(&pool, player2_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    } else {
        None
    };

    // Récupérer le round actuel
    let current_round = RankedMatchRound::get_current_round(&pool, match_data.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(MatchStatus {
        match_data: match_data.clone(),
        current_round,
        player1,
        player2,
    }))
}

/// Indique que le joueur est prêt pour le round actuel
pub async fn set_ready(
    State(pool): State<PgPool>,
        Extension(user): Extension<User>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Récupérer le match actif du joueur
    let active_matches = RankedMatch::get_active_matches_for_player(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let match_data = active_matches.first()
        .ok_or((StatusCode::NOT_FOUND, "Aucun match en cours".to_string()))?;

    // Récupérer le round actuel
    let current_round = RankedMatchRound::get_current_round(&pool, match_data.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Aucun round en cours".to_string()))?;

    // Déterminer si c'est le joueur 1 ou 2
    let player_number = if user.id == match_data.player1_id {
        1
    } else if Some(user.id) == match_data.player2_id {
        2
    } else {
        return Err((StatusCode::FORBIDDEN, "Vous n'êtes pas dans ce match".to_string()));
    };

    // Mettre à jour le statut ready
    let mut current_round = current_round;
    current_round.set_player_ready(&pool, player_number)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
} 
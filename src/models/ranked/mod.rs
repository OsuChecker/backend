use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use utoipa::ToSchema;

pub mod match_model;
pub mod round;
pub mod score;
pub mod queue;

// États du match
pub const STATUS_WAITING_PLAYER: &str = "waiting_player";
pub const STATUS_PLAYING: &str = "playing";
pub const STATUS_COMPLETED: &str = "completed";

// États d'un round
pub const ROUND_STATUS_PREPARING: &str = "preparing";
pub const ROUND_STATUS_PLAYING: &str = "playing";
pub const ROUND_STATUS_COMPLETED: &str = "completed";

// Type de match
pub const MATCH_TYPE_FIVE_MINUTES: &str = "five_minutes";

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RankedMatch {
    pub id: i32,
    pub player1_id: i32,
    pub player2_id: Option<i32>,
    pub status: String,
    pub match_type: String,
    pub mode: i32,
    pub best_of: i32,
    pub player1_points: i32,
    pub player2_points: i32,
    pub preparation_duration: i32,  // en secondes
    pub play_duration: i32,         // en secondes
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RankedMatchRound {
    pub id: i32,
    pub match_id: i32,
    pub round_number: i32,
    pub beatmap_id: i32,
    pub status: String,
    pub player1_ready: bool,
    pub player2_ready: bool,
    pub player1_best_score_id: Option<i32>,
    pub player2_best_score_id: Option<i32>,
    pub winner_id: Option<i32>,
    pub preparation_start: Option<NaiveDateTime>,
    pub play_start: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RankedMatchBannedMap {
    pub id: i32,
    pub match_id: i32,
    pub beatmap_id: i32,
    pub banned_by: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RankedMatchRoundScore {
    pub id: i32,
    pub round_id: i32,
    pub player_id: i32,
    pub score_id: i32,
    pub created_at: NaiveDateTime,
} 
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info, warn};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::models::score::score_rating::{ScoreRating, RatingType, CreateScoreRating};
use crate::models::score::score::Score;
use crate::models::map::beatmap::Beatmap;
use crate::helpers::pp::calculate_pp_for_score;
#[derive(Debug, Deserialize, IntoParams, ToSchema, Validate)]
pub struct PPCalculationParams {
    /// Nombre maximum de scores à traiter en une fois (défaut: 100, max: 1000)
    #[validate(range(min = 1, max = 1000))]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PPCalculationResponse {
    pub success: bool,
    pub message: String,
    pub scores_processed: usize,
    pub scores_failed: usize,
    pub total_scores_without_pp: usize,
}

/// Handler pour recalculer TOUS les score_rating de type "pp"
/// 
/// Ce handler :
/// 1. Récupère TOUS les scores 
/// 2. Pour chaque score, récupère la beatmap associée
/// 3. Calcule le PP en utilisant rosu-pp
/// 4. Supprime l'ancien rating PP s'il existe et crée le nouveau
#[utoipa::path(
    post,
    path = "/api/scores/calculate-pp",
    tag = "Score",
    params(PPCalculationParams),
    responses(
        (status = 200, description = "PP calculation completed", body = PPCalculationResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Internal server error")
    ),
    summary = "Recalculate ALL PP ratings",
    description = "Recalculate and recreate ALL PP ratings for scores using rosu-pp"
)]
pub async fn calculate_missing_pp(
    State(pool): State<PgPool>,
    Query(params): Query<PPCalculationParams>,
) -> Result<Json<PPCalculationResponse>, StatusCode> {
    info!("Starting PP calculation process");

    // Validation des paramètres
    if let Err(validation_errors) = params.validate() {
        error!("Invalid parameters: {:?}", validation_errors);
        return Err(StatusCode::BAD_REQUEST);
    }

    let limit = params.limit.unwrap_or(100);
    info!("Processing {} scores maximum", limit);

    // Récupérer le type de rating "pp"
    let pp_rating_type = match RatingType::get_by_name(&pool, "pp").await {
        Ok(Some(rating_type)) => rating_type,
        Ok(None) => {
            error!("Rating type 'pp' not found in database");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(e) => {
            error!("Failed to get rating type 'pp': {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Récupérer TOUS les scores (pas seulement ceux sans PP)
    let all_scores = match Score::get_all(&pool, Some(limit), Some(0)).await {
        Ok(scores) => scores,
        Err(e) => {
            error!("Failed to get scores: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let total_scores = all_scores.len();
    info!("Found {} scores to recalculate PP", total_scores);

    let mut processed_count = 0;
    let mut failed_count = 0;

    for score in all_scores {
        // Récupérer la beatmap associée
        let beatmap = match Beatmap::get_by_id(&pool, score.beatmap_id).await {
            Ok(Some(beatmap)) => beatmap,
            Ok(None) => {
                warn!("Beatmap {} not found for score {}", score.beatmap_id, score.id);
                failed_count += 1;
                continue;
            }
            Err(e) => {
                error!("Failed to get beatmap {}: {}", score.beatmap_id, e);
                failed_count += 1;
                continue;
            }
        };

        // Calculer le PP avec rosu-pp
        let calculated_pp = match calculate_pp_for_score(&score, &beatmap).await {
            Ok(pp_value) => pp_value,
            Err(e) => {
                error!("Failed to calculate PP for score {}: {}", score.id, e);
                failed_count += 1;
                continue;
            }
        };

        // Créer le score_rating
        let pp_decimal = match BigDecimal::from_f64(calculated_pp) {
            Some(decimal) => decimal,
            None => {
                error!("Failed to convert PP value {} to BigDecimal", calculated_pp);
                failed_count += 1;
                continue;
            }
        };

        let create_rating = CreateScoreRating {
            score_id: score.id,
            rating_type_id: pp_rating_type.id,
            rating_value: pp_decimal,
            max_rating: None, // TODO: Calculer max_rating si nécessaire
        };

        // Supprimer l'ancien rating PP s'il existe
        if let Ok(existing_rating) = ScoreRating::get_by_score_and_type(&pool, score.id, &pp_rating_type.name).await {
            if let Some(_) = existing_rating {
                match sqlx::query!("DELETE FROM score_rating WHERE score_id = $1 AND rating_type_id = $2", score.id, pp_rating_type.id)
                    .execute(&pool)
                    .await 
                {
                    Ok(_) => info!("Deleted existing PP rating for score {}", score.id),
                    Err(e) => warn!("Failed to delete existing PP rating for score {}: {}", score.id, e),
                }
            }
        }

        match ScoreRating::create(&pool, create_rating).await {
            Ok(_) => {
                processed_count += 1;
                info!("Created PP rating {} for score {}", calculated_pp, score.id);
            }
            Err(e) => {
                error!("Failed to create score_rating for score {}: {}", score.id, e);
                failed_count += 1;
            }
        }
    }

    let response = PPCalculationResponse {
        success: failed_count == 0,
        message: format!("Processing completed: {} scores processed, {} failed", processed_count, failed_count),
        scores_processed: processed_count,
        scores_failed: failed_count,
        total_scores_without_pp: total_scores,
    };

    info!("PP calculation process completed: {} processed, {} failed", processed_count, failed_count);
    Ok(Json(response))
}



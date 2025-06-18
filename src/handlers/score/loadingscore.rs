use crate::models::score::score::{Score, ScoreSchema};
use axum::{response::Json, http::StatusCode, extract::State};
use bytes::Bytes;
use sqlx::PgPool;
use osu_db::ScoreList;
use axum_extra::extract::multipart::Multipart;

/// Handler pour recevoir et charger des scores depuis un fichier .db en bytes
/// 
/// Reçoit les données du fichier scores.db via multipart form et les parse en mémoire
/// sans les sauvegarder en base de données
#[axum::debug_handler]
pub async fn load_scores_db(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<ScoreLoadResponse>, StatusCode> {
    // Récupérer le fichier via multipart
    let mut file_data: Option<Bytes> = None;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Erreur lors de la lecture du champ multipart: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        if field.name() == Some("file") {
            file_data = Some(field.bytes().await.map_err(|e| {
                tracing::error!("Erreur lors de la lecture des bytes: {}", e);
                StatusCode::BAD_REQUEST
            })?);
            break;  // On ne traite que le premier fichier trouvé
        }
    }
    
    // Vérifier si on a bien reçu un fichier
    let bytes = match file_data {
        Some(data) => data,
        None => {
            tracing::error!("Aucun fichier scores.db reçu");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    // Tenter de parser les bytes en ScoreList
    let scores_list = match ScoreList::from_bytes(&bytes) {
        Ok(list) => list,
        Err(e) => {
            tracing::error!("Erreur lors du parsing des scores: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    // Récupérer les informations sur les scores
    let scores_count = scores_list.beatmaps.iter().map(|beatmap| beatmap.scores.len()).sum();
    
    // Construire la réponse
    let response = ScoreLoadResponse {
        success: true,
        scores_count,
        message: format!("{} scores chargés avec succès", scores_count),
    };
    
    Ok(Json(response))
}

/// Structure de réponse pour le chargement des scores
#[derive(serde::Serialize)]
pub struct ScoreLoadResponse {
    success: bool,
    scores_count: usize,
    message: String,
}
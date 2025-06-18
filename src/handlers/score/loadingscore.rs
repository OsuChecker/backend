use crate::models::score::score::{Score, ScoreSchema};
use crate::models::score::score_stats::{ScoreStats, generate_score_stats};
use axum::{response::Json, http::StatusCode, extract::State};
use bytes::Bytes;
use sqlx::PgPool;
use osu_db::{ScoreList, Mode};
use axum_extra::extract::multipart::Multipart;
use std::collections::HashMap;
use crate::helpers::hit::calculate_accuracy;
use crate::helpers::hit::Hit;

#[derive(Debug, Default, Clone, serde::Serialize)]
struct MostPlayedBeatmap {
    beatmap_hash: String,
    count: u32,
}
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
    let bytes = extract_file_from_multipart(&mut multipart).await?;
    
    // Parser les bytes en ScoreList
    let scores_list = parse_scores_db(&bytes)?;
    
    // Générer les statistiques
    let stats = generate_score_stats(&scores_list,&pool).await;
    
    // Construire la réponse
    let response = ScoreLoadResponse {
        success: true,
        stats: stats.clone(),
        message: format!("{} scores chargés avec succès sur {} beatmaps", 
                         stats.scores_count, stats.beatmaps_count),
    };
    
    Ok(Json(response))
}

/// Extrait le fichier depuis la requête multipart
async fn extract_file_from_multipart(multipart: &mut Multipart) -> Result<Bytes, StatusCode> {
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
    match file_data {
        Some(data) => Ok(data),
        None => {
            tracing::error!("Aucun fichier scores.db reçu");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Parse les bytes en ScoreList
fn parse_scores_db(bytes: &Bytes) -> Result<ScoreList, StatusCode> {
    match ScoreList::from_bytes(bytes) {
        Ok(list) => Ok(list),
        Err(e) => {
            tracing::error!("Erreur lors du parsing des scores: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Structure de réponse pour le chargement des scores
#[derive(serde::Serialize)]
pub struct ScoreLoadResponse {
    success: bool,
    stats: ScoreStats,
    message: String,
}

#[derive(serde::Serialize)]
pub struct HitStats {
    count_300: u32,
    count_100: u32,
    count_50: u32,
    count_miss: u32,
    count_geki: u32,
    count_katu: u32,
    accuracy: f64,
}

#[derive(serde::Serialize)]
pub struct ComboStats {
    max_combo_ever: u16,
    perfect_combos: usize,
}

#[derive(serde::Serialize)]
pub struct ScoreHighlight {
    score: u32,
    player: String,
}
use std::collections::HashMap;
use bigdecimal::FromPrimitive;
use serde::Serialize;
use osu_db::{ScoreList, Mode};
use crate::helpers::hit::{Hit, calculate_accuracy};
use crate::models::map::beatmap::Beatmap;
use crate::models::score::score::{Score, CreateScore, ScoreStatistics};
use sqlx::PgPool;
use crate::helpers::hit::accuracy_to_rank;
use sqlx::types::BigDecimal;
use bigdecimal::ToPrimitive;
use crate::models::user::user::User;

/// Structure pour les statistiques de scores
#[derive(Debug, Serialize, Clone)]
pub struct ScoreStats {
    pub scores_count: usize,
    pub beatmaps_count: usize,
    pub mode_stats: HashMap<String, usize>,
    pub hit_stats: HitStats,
    pub combo_stats: ComboStats,
    pub top_score: ScoreHighlight,
    pub most_played_beatmap: MostPlayedBeatmap,
}

#[derive(Debug, Serialize, Clone)]
pub struct HitStats {
    pub count_300: u32,
    pub count_100: u32,
    pub count_50: u32,
    pub count_miss: u32,
    pub count_geki: u32,
    pub count_katu: u32,
    pub accuracy: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct ComboStats {
    pub max_combo_ever: u16,
    pub perfect_combos: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct ScoreHighlight {
    pub score: u32,
    pub player: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct MostPlayedBeatmap {
    pub beatmap_hash: String,
    pub count: u32,
}

/// Génère des statistiques à partir d'une liste de scores
pub async fn generate_score_stats(scores_list: &ScoreList, pool: &PgPool) -> ScoreStats {
    // Récupérer les statistiques détaillées
    let total_scores_count = scores_list.beatmaps.iter().map(|beatmap| beatmap.scores.len()).sum();
    let total_beatmaps = scores_list.beatmaps.len();
    
    // Statistiques par mode de jeu
    let mut scores_by_mode: HashMap<Mode, usize> = HashMap::new();
    let mut hit = Hit::new(0, 0, 0, 0, 0, 0);
    let mut max_combo_ever = 0;
    let mut total_perfect_combos = 0;
    let mut top_score = 0;
    let mut top_player = String::new();
    let mut most_played_beatmap = MostPlayedBeatmap {
        beatmap_hash: String::new(),
        count: 0,
    };

    for beatmap in &scores_list.beatmaps {
        let beatmap_hash = beatmap.hash.clone().unwrap_or_else(|| String::new());
        let bm: Option<Beatmap> = Beatmap::get_beatmap_by_hash(&pool, &beatmap_hash).await.unwrap();
        if beatmap.scores.len() > most_played_beatmap.count as usize {
            most_played_beatmap.beatmap_hash = beatmap.hash.clone().unwrap_or_else(|| String::new());
            most_played_beatmap.count = beatmap.scores.len() as u32;
        }
        
        for score in &beatmap.scores {
            let acc = calculate_accuracy(3, Hit{_300: score.count_300 as u32, _100: score.count_100 as u32, _50: score.count_50 as u32, _miss: score.count_miss as u32, _geki: score.count_geki as u32, _katu: score.count_katsu as u32});
            let acc = BigDecimal::from_f64(acc).unwrap();
            let user = User::get_by_username(&pool, score.player_name.as_ref().unwrap()).await.unwrap();
            let create_score = CreateScore {
                user_id: user.unwrap().id,
                beatmap_id: bm.clone().unwrap().id,
                score: score.score as i32,
                max_combo: score.max_combo as i32,
                perfect: score.perfect_combo,
                statistics: ScoreStatistics {
                    count_300: score.count_300 as i32,
                    count_100: score.count_100 as i32,
                    count_50: score.count_50 as i32,
                    count_miss: score.count_miss as i32,
                    count_geki: score.count_geki as i32,
                    count_katu: score.count_katsu as i32,
                },
                mods: score.mods.bits() as i32,
                accuracy: acc.clone(),
                rank: accuracy_to_rank(acc.clone().to_f64().unwrap()),
                replay_available:false,
            };
            Score::create(pool, create_score).await;
            // Compter par mode
            *scores_by_mode.entry(score.mode).or_insert(0) += 1;
            
            // Statistiques globales
            hit._300 += score.count_300 as u32;
            hit._100 += score.count_100 as u32;
            hit._50 += score.count_50 as u32;
            hit._miss += score.count_miss as u32;
            hit._geki += score.count_geki as u32;
            hit._katu += score.count_katsu as u32;
            
            // Meilleur combo
            if score.max_combo > max_combo_ever {
                max_combo_ever = score.max_combo;
            }
            
            // Combos parfaits
            if score.perfect_combo {
                total_perfect_combos += 1;
            }
            
            // Meilleur score
            if score.score > top_score {
                top_score = score.score;
                top_player = score.player_name.clone().unwrap_or_else(|| "Unknown".to_string());
            }
        }
    }
    
    // Calculer les pourcentages de précision
    let accuracy = calculate_accuracy(3, hit.clone()) * 100.0;
    
    // Construire la réponse
    ScoreStats {
        scores_count: total_scores_count,
        beatmaps_count: total_beatmaps,
        mode_stats: scores_by_mode.into_iter()
            .map(|(mode, count)| (format!("{:?}", mode), count))
            .collect(),
        hit_stats: HitStats {
            count_300: hit._300,
            count_100: hit._100,
            count_50: hit._50,
            count_miss: hit._miss,
            count_geki: hit._geki,
            count_katu: hit._katu,
            accuracy,
        },
        combo_stats: ComboStats {
            max_combo_ever,
            perfect_combos: total_perfect_combos,
        },
        top_score: ScoreHighlight {
            score: top_score,
            player: top_player,
        },
        most_played_beatmap,
    }
} 
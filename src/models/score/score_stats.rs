use std::collections::HashMap;
use serde::Serialize;
use osu_db::{ScoreList, Mode};
use crate::helpers::hit::{Hit, calculate_accuracy, accuracy_to_rank};
use sqlx::PgPool;
use crate::models::map::beatmap_queue::BeatmapQueue;
use crate::models::score::score::{CreateScore, ScoreStatistics};
use crate::models::user::user::{User, CreateUser};
use sqlx::types::BigDecimal;
use bigdecimal::FromPrimitive;
use tracing::error;

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
pub async fn generate_score_stats(scores_list: &ScoreList, username: &str,  pool: &PgPool) -> ScoreStats {
    let total_scores_count = scores_list.beatmaps.iter()
        .map(|beatmap| beatmap.scores.len())
        .sum();
    let total_beatmaps = scores_list.beatmaps.len();
    
    // Initialisation des statistiques
    let mut stats = StatsAccumulator::new();
    
    // Traitement des beatmaps et scores
    'outer: for beatmap in &scores_list.beatmaps {
        let Some(hash) = &beatmap.hash else { continue };
        
        // Mise à jour de la beatmap la plus jouée
        if beatmap.scores.len() > stats.most_played_beatmap.count as usize {
            stats.most_played_beatmap = MostPlayedBeatmap {
                beatmap_hash: hash.clone(),
                count: beatmap.scores.len() as u32,
            };
        }
        
        // Traitement des scores de cette beatmap
        for score in &beatmap.scores {
            
            // Traitement du score
            if let Err(e) = process_single_score(score, hash, pool, &mut stats, username).await {
                error!("Erreur lors du traitement du score: {}", e);
                continue;
            }
        }
    }
    
    // Construction des statistiques finales
    stats.build_final_stats(total_scores_count, total_beatmaps)
}

/// Accumulateur pour les statistiques
struct StatsAccumulator {
    scores_by_mode: HashMap<Mode, usize>,
    hit: Hit,
    max_combo_ever: u16,
    perfect_combos: usize,
    top_score: u32,
    top_player: String,
    most_played_beatmap: MostPlayedBeatmap,
}

impl StatsAccumulator {
    fn new() -> Self {
        Self {
            scores_by_mode: HashMap::new(),
            hit: Hit::new(0, 0, 0, 0, 0, 0),
            max_combo_ever: 0,
            perfect_combos: 0,
            top_score: 0,
            top_player: String::new(),
            most_played_beatmap: MostPlayedBeatmap::default(),
        }
    }
    
    fn update_with_score(&mut self, score: &osu_db::Replay) {
        // Compter par mode
        *self.scores_by_mode.entry(score.mode).or_insert(0) += 1;
        
        // Statistiques de hits
        self.hit._300 += score.count_300 as u32;
        self.hit._100 += score.count_100 as u32;
        self.hit._50 += score.count_50 as u32;
        self.hit._miss += score.count_miss as u32;
        self.hit._geki += score.count_geki as u32;
        self.hit._katu += score.count_katsu as u32;
        
        // Meilleur combo
        if score.max_combo > self.max_combo_ever {
            self.max_combo_ever = score.max_combo;
        }
        
        // Combos parfaits
        if score.perfect_combo {
            self.perfect_combos += 1;
        }
        
        // Meilleur score
        if score.score > self.top_score {
            self.top_score = score.score;
            self.top_player = score.player_name
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());
        }
    }
    
    fn build_final_stats(self, total_scores: usize, total_beatmaps: usize) -> ScoreStats {
        let accuracy = calculate_accuracy(3, self.hit.clone()) * 100.0;
        
        ScoreStats {
            scores_count: total_scores,
            beatmaps_count: total_beatmaps,
            mode_stats: self.scores_by_mode.into_iter()
                .map(|(mode, count)| (format!("{:?}", mode), count))
                .collect(),
            hit_stats: HitStats {
                count_300: self.hit._300,
                count_100: self.hit._100,
                count_50: self.hit._50,
                count_miss: self.hit._miss,
                count_geki: self.hit._geki,
                count_katu: self.hit._katu,
                accuracy,
            },
            combo_stats: ComboStats {
                max_combo_ever: self.max_combo_ever,
                perfect_combos: self.perfect_combos,
            },
            top_score: ScoreHighlight {
                score: self.top_score,
                player: self.top_player,
            },
            most_played_beatmap: self.most_played_beatmap,
        }
    }
}

/// Traite un score individuel
async fn process_single_score(
    score: &osu_db::Replay,
    beatmap_hash: &str,
    pool: &PgPool,
    stats: &mut StatsAccumulator,
    username: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    // Mise à jour des statistiques
    stats.update_with_score(score);
    
    // Gestion de l'utilisateur
    let user = get_or_create_user(pool, username).await?;
    
    // Création du score pour la queue
    let create_score = build_create_score(score, beatmap_hash, user.id)?;
    
    // Ajout à la queue
    BeatmapQueue::add_score(create_score).await?;
    
    Ok(())
}

/// Récupère ou crée un utilisateur
async fn get_or_create_user(
    pool: &PgPool,
    player_name: &str,
) -> Result<crate::models::user::user::User, Box<dyn std::error::Error + Send + Sync>> {
    let test_username = format!("{}test", player_name);
    
    match User::get_by_username(pool, &test_username).await? {
        Some(user) => Ok(user),
        None => {
            let create_user = CreateUser {
                username: test_username.clone(),
                email: format!("{}@test.com", test_username),
                password: String::new(),
                country: String::new(),
            };
            Ok(User::create(pool, create_user).await?)
        }
    }
}

/// Construit un CreateScore à partir des données osu_db
fn build_create_score(
    score: &osu_db::Replay,
    beatmap_hash: &str,
    user_id: i32,
) -> Result<CreateScore, Box<dyn std::error::Error + Send + Sync>> {
    let score_hit = Hit {
        _300: score.count_300 as u32,
        _100: score.count_100 as u32,
        _50: score.count_50 as u32,
        _miss: score.count_miss as u32,
        _geki: score.count_geki as u32,
        _katu: score.count_katsu as u32,
    };
    
    let accuracy = calculate_accuracy(score.mode as u8, score_hit);
    let accuracy_decimal = BigDecimal::from_f64(accuracy)
        .ok_or("Failed to convert accuracy to BigDecimal")?;
    
    Ok(CreateScore {
        user_id,
        beatmap_hash: beatmap_hash.to_string(),
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
        accuracy: accuracy_decimal,
        rank: accuracy_to_rank(accuracy),
        replay_available: false,
        hash: score.replay_hash.clone(),
    })
} 
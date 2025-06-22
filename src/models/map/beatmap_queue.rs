use sqlx::PgPool;
use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::{Result, Context};
use tokio::time;
use once_cell::sync::Lazy;
use crate::helpers::osuapi::{OsuAPI, BeatmapResponse};
use crate::models::map::beatmap::{Beatmap, CreateBeatmap};
use crate::models::map::beatmapset::{Beatmapset, CreateBeatmapset};
use crate::models::score::score::{Score, CreateScore};
use sqlx::types::BigDecimal;
use tracing::{error, warn};
use std::collections::HashMap;
use flume::{Sender, Receiver};
use tokio::sync::Mutex;

// Structure pour les requêtes de beatmap
#[derive(Debug, Clone)]
pub struct BeatmapRequest {
    pub hash: String,
    pub scores: Vec<CreateScore>,
}

// Singleton pour la file d'attente des beatmaps avec flume
static BEATMAP_QUEUE: Lazy<(Sender<BeatmapRequest>, Receiver<BeatmapRequest>)> = 
    Lazy::new(|| flume::unbounded());

// Limites de l'API
const RATE_LIMIT: usize = 150; // 150 requêtes par minute
const RATE_WINDOW: Duration = Duration::from_secs(60);

pub struct BeatmapQueue;

impl BeatmapQueue {
    /// Initialise la file d'attente des beatmaps
    pub async fn init(pool: PgPool, api: OsuAPI) -> Result<()> {
        let receiver = BEATMAP_QUEUE.1.clone();
        
        // Lancer le worker en tâche de fond
        tokio::spawn(async move {
            let mut request_times = Vec::with_capacity(RATE_LIMIT);
            
            while let Ok(request) = receiver.recv_async().await {
                // Limiter le débit des requêtes
                Self::enforce_rate_limit(&mut request_times).await;
                
                // Traiter la requête
                match Self::process_beatmap_request(&request, &pool, &api).await {
                    Ok(beatmap_id) => {
                        // Traiter les scores maintenant que la beatmap existe
                        Self::process_scores(&request.scores, beatmap_id, &pool).await;
                    }
                    Err(e) => {
                        error!("Worker: Échec du traitement de la beatmap {}: {}", request.hash, e);
                    }
                }
            }
            
            error!("Worker: Boucle de traitement terminée - le canal a été fermé");
        });
        
        Ok(())
    }
    
    /// Ajoute un score à traiter
    pub async fn add_score(score: CreateScore) -> Result<()> {
        let hash = score.beatmap_hash.clone();
        
        // Créer une requête avec ce score
        let request = BeatmapRequest {
            hash: hash.clone(),
            scores: vec![score],
        };
        
        // Envoyer à la queue
        BEATMAP_QUEUE.0.send_async(request).await
            .map_err(|e| anyhow::anyhow!("Failed to send to queue: {}", e))?;
        
        Ok(())
    }
    
    /// Limite le débit des requêtes API
    async fn enforce_rate_limit(request_times: &mut Vec<Instant>) {
        let now = Instant::now();
        
        // Supprimer les requêtes plus anciennes que la fenêtre de temps
        request_times.retain(|&time| now.duration_since(time) < RATE_WINDOW);
        
        // Si on a atteint la limite, attendre
        if request_times.len() >= RATE_LIMIT {
            let oldest = request_times[0];
            let wait_time = RATE_WINDOW - now.duration_since(oldest);
            if wait_time > Duration::from_millis(0) {
                warn!("Rate limit reached, waiting for {:?}", wait_time);
                time::sleep(wait_time).await;
            }
        }
        
        // Ajouter cette requête
        request_times.push(Instant::now());
    }
    
    /// Traite une requête de beatmap
    async fn process_beatmap_request(
        request: &BeatmapRequest, 
        pool: &PgPool, 
        api: &OsuAPI
    ) -> Result<i32> {
        let hash = &request.hash;
        
        // Vérifier si la beatmap existe déjà
        if let Ok(Some(beatmap)) = Beatmap::get_by_md5(pool, hash).await {
            return Ok(beatmap.id);
        }
        
        // Récupérer les données depuis l'API osu!
        let beatmap_data = api.get_beatmap_by_md5(hash).await
            .context("Failed to get beatmap from API")?;
        
        // Créer le beatmapset s'il n'existe pas
        let beatmapset_id = Self::ensure_beatmapset_exists(pool, &beatmap_data).await
            .context("Failed to ensure beatmapset exists")?;
        
        // Créer la beatmap
        let beatmap = Self::create_beatmap(pool, &beatmap_data, beatmapset_id).await
            .context("Failed to create beatmap")?;
        
        Ok(beatmap.id)
    }
    
    /// Traite les scores pour une beatmap
    async fn process_scores(scores: &[CreateScore], beatmap_id: i32, pool: &PgPool) {
        let mut error_count = 0;
        
        for score in scores {
            if let Err(e) = Score::create(pool, score.clone()).await {
                error_count += 1;
                error!("Échec de création du score pour la beatmap {}: {}", beatmap_id, e);
            }
        }
    }
    
    /// Assure que le beatmapset existe, le crée si nécessaire
    async fn ensure_beatmapset_exists(pool: &PgPool, beatmap_data: &BeatmapResponse) -> Result<i32> {
        // Vérifier si le beatmapset existe déjà
        if let Ok(Some(beatmapset)) = Beatmapset::get_by_osu_id(pool, beatmap_data.beatmapset.id).await {
            return Ok(beatmapset.id);
        }
        
        let create_beatmapset = CreateBeatmapset {
            osu_id: Some(beatmap_data.beatmapset.id),
            artist: beatmap_data.beatmapset.artist.clone(),
            artist_unicode: Some(beatmap_data.beatmapset.artist_unicode.clone()),
            title: beatmap_data.beatmapset.title.clone(),
            title_unicode: Some(beatmap_data.beatmapset.title_unicode.clone()),
            creator_id: None,
            source: Some(beatmap_data.beatmapset.source.clone()),
            tags: Some(beatmap_data.beatmapset.tags.split_whitespace().map(String::from).collect()),
            status: beatmap_data.status.clone(),
            has_video: beatmap_data.beatmapset.video,
            has_storyboard: beatmap_data.beatmapset.storyboard,
            is_explicit: beatmap_data.beatmapset.nsfw,
            is_featured: beatmap_data.beatmapset.spotlight,
            cover_url: Some(beatmap_data.beatmapset.covers.cover.clone()),
            preview_url: Some(beatmap_data.beatmapset.preview_url.clone()),
            osu_file_url: None,
        };
        
        let beatmapset = Beatmapset::create(pool, create_beatmapset).await
            .context("Failed to create beatmapset")?;
        
        Ok(beatmapset.id)
    }
    
    /// Crée une beatmap à partir des données de l'API
    async fn create_beatmap(pool: &PgPool, response: &BeatmapResponse, beatmapset_id: i32) -> Result<Beatmap, sqlx::Error> {
        let create_beatmap = CreateBeatmap {
            beatmapset_id,
            version: response.version.clone(),
            difficulty_rating: BigDecimal::try_from(response.difficulty_rating).unwrap_or_default(),
            count_circles: response.count_circles,
            count_sliders: response.count_sliders,
            count_spinners: response.count_spinners,
            max_combo: response.max_combo.unwrap_or(0),
            drain_time: response.drain as i32,
            total_time: response.total_length,
            bpm: BigDecimal::try_from(response.bpm).unwrap_or_default(),
            cs: BigDecimal::try_from(response.cs).unwrap_or_default(),
            ar: BigDecimal::try_from(response.ar).unwrap_or_default(),
            od: BigDecimal::try_from(response.accuracy).unwrap_or_default(),
            hp: BigDecimal::try_from(response.drain).unwrap_or_default(),
            mode: response.mode_int,
            status: response.status.clone(),
            hit_length: response.hit_length,
            file_md5: response.checksum.clone().unwrap_or_default(),
            file_path: format!("beatmaps/{}.osu", response.id),
        };
        
        Beatmap::create(pool, create_beatmap).await
    }
} 
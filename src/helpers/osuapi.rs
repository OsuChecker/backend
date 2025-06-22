use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};
use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;
use tracing::{info, warn, error, debug};

// Structure pour stocker les tokens OAuth
#[derive(Debug, Clone, Deserialize)]
struct OAuthTokens {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

// Structure pour la réponse de l'API beatmap
#[derive(Debug, Clone, Deserialize)]
pub struct BeatmapResponse {
    pub id: i32,
    pub beatmapset_id: i32,
    pub mode: String,
    pub mode_int: i32,
    pub status: String,
    pub version: String,
    pub difficulty_rating: f64,
    pub cs: f64,
    pub ar: f64,
    pub accuracy: f64,
    pub drain: f64,
    pub total_length: i32,
    pub hit_length: i32,
    pub bpm: f64,
    pub checksum: Option<String>,
    pub max_combo: Option<i32>,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub user_id: i32,
    pub beatmapset: BeatmapsetResponse,
}

// Structure pour la partie beatmapset de la réponse
#[derive(Debug, Clone, Deserialize)]
pub struct BeatmapsetResponse {
    pub id: i32,
    pub artist: String,
    pub artist_unicode: String,
    pub title: String,
    pub title_unicode: String,
    pub creator: String,
    pub user_id: i32,
    pub source: String,
    pub tags: String,
    pub video: bool,
    pub storyboard: bool,
    pub nsfw: bool,
    pub spotlight: bool,
    pub preview_url: String,
    pub covers: BeatmapsetCovers,
}

// Structure pour les covers du beatmapset
#[derive(Debug, Clone, Deserialize)]
pub struct BeatmapsetCovers {
    pub cover: String,
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    pub card: String,
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    pub list: String,
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    pub slimcover: String,
    #[serde(rename = "slimcover@2x")]
    pub slimcover_2x: String,
}

// Cache simple pour le token
struct TokenCache {
    access_token: Option<String>,
    expires_at: Option<u64>,
}

// Singleton pour le cache de token
static TOKEN_CACHE: Lazy<Arc<Mutex<TokenCache>>> = Lazy::new(|| {
    Arc::new(Mutex::new(TokenCache {
        access_token: None,
        expires_at: None,
    }))
});

pub struct OsuAPI {
    client: Client,
    client_id: String,
    client_secret: String,
}

impl OsuAPI {
    pub fn new(client_id: String, client_secret: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            client_id,
            client_secret,
        }
    }

    /// Récupère un token d'accès (en créant un nouveau ou en utilisant celui en cache)
    pub async fn get_access_token(&self) -> Result<String> {
        let mut cache = TOKEN_CACHE.lock().await;
        
        // Vérifier si le token est valide
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Si le token est expiré ou n'existe pas, en demander un nouveau
        if cache.access_token.is_none() || 
           cache.expires_at.is_none() || 
           current_time >= cache.expires_at.unwrap() {
            debug!("Token OAuth expiré ou inexistant, demande d'un nouveau token");
            let tokens = self.request_access_token().await?;
            
            // Mettre à jour le cache
            cache.access_token = Some(tokens.access_token);
            cache.expires_at = Some(current_time + tokens.expires_in - 60); // Marge de sécurité de 60s
            
            info!("Nouveau token OAuth obtenu, valide pour {} secondes", tokens.expires_in);
        } else {
            let remaining = cache.expires_at.unwrap() - current_time;
            debug!("Utilisation du token OAuth en cache (expire dans {} secondes)", remaining);
        }

        Ok(cache.access_token.clone().unwrap())
    }

    /// Demande un nouveau token d'accès
    async fn request_access_token(&self) -> Result<OAuthTokens> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "client_credentials"),
            ("scope", "public"),
        ];

        info!("Demande d'un nouveau token OAuth à l'API osu!");
        
        let response = self.client
            .post("https://osu.ppy.sh/oauth/token")
            .header(header::ACCEPT, "application/json")
            .form(&params)
            .send()
            .await
            .context("Failed to request access token")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Impossible de lire le corps de la réponse".to_string());
            error!("Échec de l'authentification OAuth: HTTP {}, Réponse: {}", status, error_text);
            return Err(anyhow::anyhow!(
                "Failed to get access token: HTTP {}, Response: {}", 
                status, error_text
            ));
        }

        let tokens: OAuthTokens = response
            .json()
            .await
            .context("Failed to parse token response")?;

        debug!("Token OAuth obtenu avec succès: type={}, expire_dans={}s", tokens.token_type, tokens.expires_in);
        Ok(tokens)
    }

    /// Effectue une requête à l'API osu!
    pub async fn api_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T> {
        let access_token = self.get_access_token().await?;
        
        let url = format!("https://osu.ppy.sh/api/v2/{}", endpoint);
        
        // Construire une chaîne de requête lisible pour les logs
        let query_string = params.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        info!("Requête API osu!: GET {} ?{}", url, query_string);
        
        let response = self.client
            .get(&url)
            .header(
                header::AUTHORIZATION, 
                format!("Bearer {}", access_token)
            )
            .query(params)
            .send()
            .await
            .context("Failed to make API request")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Impossible de lire le corps de la réponse".to_string());
            error!("Échec de la requête API: HTTP {}, URL: {}, Réponse: {}", status, url, error_text);
            return Err(anyhow::anyhow!(
                "API request failed: HTTP {}, URL: {}, Response: {}", 
                status, url, error_text
            ));
        }

        debug!("Réponse API reçue: HTTP {}", status);
        
        let data = response
            .json::<T>()
            .await
            .context("Failed to parse API response")?;

        info!("Requête API osu! réussie: {} ?{}", url, query_string);
        Ok(data)
    }

    /// Récupère une beatmap par son hash MD5
    pub async fn get_beatmap_by_md5(&self, md5: &str) -> Result<BeatmapResponse> {
        info!("Recherche de beatmap par MD5: {}", md5);
        info!("Utilisation de l'API avec client_id: {}", self.client_id);
        
        let result = self.api_request::<BeatmapResponse>(
            "beatmaps/lookup", 
            &[("checksum", md5)]
        ).await;
        
        match &result {
            Ok(beatmap) => info!("Beatmap trouvée: id={}, titre={}, version={}", beatmap.id, beatmap.beatmapset.title, beatmap.version),
            Err(e) => {
                error!("Échec de la recherche de beatmap avec le MD5 {}: {}", md5, e);
                error!("Détails de l'erreur: {:?}", e);
            }
        }
        
        result
    }

    /// Récupère une beatmap par son ID
    pub async fn get_beatmap_by_id(&self, id: &str) -> Result<BeatmapResponse> {
        info!("Recherche de beatmap par ID: {}", id);
        let result = self.api_request::<BeatmapResponse>(
            "beatmaps/lookup", 
            &[("id", id)]
        ).await;
        
        match &result {
            Ok(beatmap) => info!("Beatmap trouvée: id={}, titre={}, version={}", beatmap.id, "Unknown", beatmap.version),
            Err(e) => warn!("Beatmap non trouvée avec l'ID {}: {}", id, e),
        }
        
        result
    }
} 
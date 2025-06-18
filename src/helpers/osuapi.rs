use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};
use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;

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
            let tokens = self.request_access_token().await?;
            
            // Mettre à jour le cache
            cache.access_token = Some(tokens.access_token);
            cache.expires_at = Some(current_time + tokens.expires_in - 60); // Marge de sécurité de 60s
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

        let response = self.client
            .post("https://osu.ppy.sh/oauth/token")
            .header(header::ACCEPT, "application/json")
            .form(&params)
            .send()
            .await
            .context("Failed to request access token")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to get access token: HTTP {}", 
                response.status()
            ));
        }

        let tokens: OAuthTokens = response
            .json()
            .await
            .context("Failed to parse token response")?;

        Ok(tokens)
    }

    /// Effectue une requête à l'API osu!
    pub async fn api_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T> {
        let access_token = self.get_access_token().await?;
        
        let url = format!("https://osu.ppy.sh/api/v2/{}", endpoint);
        
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

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "API request failed: HTTP {}", 
                response.status()
            ));
        }

        let data = response
            .json::<T>()
            .await
            .context("Failed to parse API response")?;

        Ok(data)
    }

    /// Récupère une beatmap par son hash MD5
    pub async fn get_beatmap_by_md5(&self, md5: &str) -> Result<BeatmapResponse> {
        self.api_request::<BeatmapResponse>(
            "beatmaps/lookup", 
            &[("checksum", md5)]
        ).await
    }

    /// Récupère une beatmap par son ID
    pub async fn get_beatmap_by_id(&self, id: &str) -> Result<BeatmapResponse> {
        self.api_request::<BeatmapResponse>(
            "beatmaps/lookup", 
            &[("id", id)]
        ).await
    }
} 
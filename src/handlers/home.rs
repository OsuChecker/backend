//! # Home Handler Module
//!
//! Ce module contient le handler pour la page d'accueil du serveur.

use axum::{
    response::Html,
    http::StatusCode,
};

/// Handler pour la page d'accueil
pub async fn home() -> Result<Html<&'static str>, StatusCode> {
    let html = include_str!("../../public/home.html");
    Ok(Html(html))
} 
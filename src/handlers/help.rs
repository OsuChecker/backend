//! # Help Handlers Module
//!
//! Ce module contient les handlers pour les routes d'aide et de diagnostic de l'API.
//! Ces handlers permettent de vérifier l'état de santé du système et d'obtenir
//! des informations utiles pour le debugging et le monitoring.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use sysinfo::{Disks, System};
use std::time::Instant;

use crate::{
    db::DatabaseManager,
    models::help::{
        HealthResponse, DatabaseStatus, SystemMetrics,
        PerformanceMetrics, InfoResponse, EndpointInfo,
    },
};

#[utoipa::path(
    get,
    path = "/api/help/health",
    tag = "System",
    responses(
        (status = 200, description = "System is healthy", body = HealthResponse),
        (status = 503, description = "System is unhealthy")
    ),
    summary = "Get system health status",
    description = "Performs a comprehensive health check of the system including database connection, system metrics, and performance metrics."
)]
pub async fn health_check(State(db): State<DatabaseManager>) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = Instant::now();
    
    // Vérification de la base de données
    let db_status = check_database_health(&db).await;
    
    // Métriques système
    let system_metrics = get_system_metrics();
    
    // Métriques de performance
    let response_time = start_time.elapsed().as_millis() as u64;
    let performance_metrics = PerformanceMetrics {
        response_time_ms: response_time,
    };
    
    let health_response = HealthResponse {
        status: if db_status.connected { "healthy".to_string() } else { "unhealthy".to_string() },
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        system: system_metrics,
        performance: performance_metrics,
    };
    
    if health_response.status == "healthy" {
        Ok(Json(health_response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[utoipa::path(
    get,
    path = "/api/help/health-light",
    tag = "System",
    responses(
        (status = 200, description = "System is healthy", body = HealthResponse),
        (status = 503, description = "System is unhealthy")
    ),
    summary = "Get light system health status",
    description = "Performs a quick health check focusing only on database connection and basic performance metrics."
)]
pub async fn health_light(State(db): State<DatabaseManager>) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = Instant::now();
    
    // Vérification de la base de données seulement
    let db_status = check_database_health(&db).await;
    
    // Métriques système minimales
    let system_metrics = SystemMetrics {
        cpu_usage: 0.0, // Skip CPU check for speed
        cpu_count: 0,
        memory_used_mb: 0,
        memory_total_mb: 0,
        memory_usage_percent: 0.0,
        disk_usage_percent: 0.0,
        uptime: System::uptime(),
    };
    
    // Métriques de performance
    let response_time = start_time.elapsed().as_millis() as u64;
    let performance_metrics = PerformanceMetrics {
        response_time_ms: response_time,
    };
    
    let health_response = HealthResponse {
        status: if db_status.connected { "healthy".to_string() } else { "unhealthy".to_string() },
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        system: system_metrics,
        performance: performance_metrics,
    };
    
    if health_response.status == "healthy" {
        Ok(Json(health_response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[utoipa::path(
    get,
    path = "/api/help/info",
    tag = "System",
    responses(
        (status = 200, description = "API information retrieved successfully", body = InfoResponse)
    ),
    summary = "Get API information",
    description = "Retrieves general information about the API including version, description, and available endpoints."
)]
pub async fn info() -> Json<InfoResponse> {
    Json(InfoResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").split(':').map(|s| s.trim().to_string()).collect(),
        endpoints: vec![
            EndpointInfo {
                path: "/help/health".to_string(),
                method: "GET".to_string(),
                description: "Vérification complète de l'état de santé du système".to_string(),
            },
            EndpointInfo {
                path: "/help/health-light".to_string(),
                method: "GET".to_string(),
                description: "Vérification rapide (DB + performance seulement)".to_string(),
            },
            EndpointInfo {
                path: "/help/info".to_string(),
                method: "GET".to_string(),
                description: "Informations sur l'API".to_string(),
            },
            EndpointInfo {
                path: "/help/ping".to_string(),
                method: "GET".to_string(),
                description: "Test de connectivité simple".to_string(),
            },
        ],
    })
}

#[utoipa::path(
    get,
    path = "/api/help/ping",
    tag = "System",
    responses(
        (status = 200, description = "API is reachable", body = String)
    ),
    summary = "Ping the API",
    description = "Simple endpoint to check if the API is reachable. Returns 'pong' if successful."
)]
pub async fn ping() -> &'static str {
    "pong"
}

/// Vérification de l'état de la base de données
async fn check_database_health(db: &DatabaseManager) -> DatabaseStatus {
    let start_time = Instant::now();
    
    match sqlx::query("SELECT 1 as test")
        .fetch_one(db.get_pool())
        .await
    {
        Ok(_) => DatabaseStatus {
            connected: true,
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error: None,
        },
        Err(e) => DatabaseStatus {
            connected: false,
            response_time_ms: None,
            error: Some(e.to_string()),
        },
    }
}

/// Collecte des métriques système (optimisée)
fn get_system_metrics() -> SystemMetrics {
    let mut sys = System::new();
    
    // Refresh seulement la mémoire et CPU (plus rapide)
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    
    // CPU usage (moyenne de tous les cœurs)
    let cpu_usage = if !sys.cpus().is_empty() {
        sys.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / sys.cpus().len() as f32
    } else {
        0.0
    };
    
    let cpu_count = sys.cpus().len();
    
    // Mémoire
    let memory_used = sys.used_memory() / 1024 / 1024; // Convert to MB
    let memory_total = sys.total_memory() / 1024 / 1024; // Convert to MB
    let memory_usage_percent = if memory_total > 0 {
        (memory_used as f32 / memory_total as f32) * 100.0
    } else {
        0.0
    };
    
    // Disques - seulement pour le premier disque (plus rapide)
    let disks = Disks::new_with_refreshed_list();
    let disk_usage_percent = if let Some(disk) = disks.first() {
        let total = disk.total_space();
        let available = disk.available_space();
        if total > 0 {
            let used = total - available;
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };
    
    SystemMetrics {
        cpu_usage,
        cpu_count,
        memory_used_mb: memory_used,
        memory_total_mb: memory_total,
        memory_usage_percent,
        disk_usage_percent,
        uptime: System::uptime(),
    }
} 


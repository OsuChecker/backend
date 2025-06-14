//! # Help Models Module
//!
//! Ce module contient les structures de données utilisées pour les endpoints d'aide et de diagnostic.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub database: DatabaseStatus,
    pub system: SystemMetrics,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseStatus {
    pub connected: bool,
    pub response_time_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub cpu_count: usize,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub uptime: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PerformanceMetrics {
    pub response_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
} 
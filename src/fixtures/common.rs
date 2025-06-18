use sqlx::{Pool, Postgres};
use tracing::{info, warn};
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use serde_json::Value as JsonValue;

pub struct FixtureManager {
    pool: Pool<Postgres>,
}

impl FixtureManager {
    /// Crée une nouvelle instance de FixtureManager
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Vérifie si les migrations sont à jour
    async fn check_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }

    fn validate_decimal_value(value: &str) -> Result<BigDecimal, sqlx::Error> {
        BigDecimal::from_str(value)
            .map_err(|e| sqlx::Error::Protocol(format!("Invalid decimal value: {}", e)))
    }

    fn validate_json_value(value: &JsonValue) -> Result<JsonValue, sqlx::Error> {
        Ok(value.clone())
    }

    pub async fn submit_fixtures<T: serde::Serialize>(
        &self,
        fixture_data: Vec<T>,
        table_name: &str,
    ) -> Result<(), sqlx::Error> {
        // Vérifie d'abord si les migrations sont à jour
        self.check_migrations().await?;

        let fixture_len = fixture_data.len();
        info!("Submitting {} fixtures to table {}", fixture_len.clone(), table_name);

        // Démarre une transaction
        let mut tx = self.pool.begin().await?;

        for data in fixture_data {
            // Convertit les données en JSON
            let json_data = match serde_json::to_value(data) {
                Ok(value) => value,
                Err(e) => return Err(sqlx::Error::Protocol(format!("JSON serialization error: {}", e))),
            };
            
            // Construit la requête d'insertion dynamiquement
            let columns: Vec<String> = json_data.as_object()
                .ok_or_else(|| sqlx::Error::Protocol("Invalid JSON object".into()))?
                .keys()
                .cloned()
                .collect();

            let placeholders: Vec<String> = (1..=columns.len())
                .map(|i| format!("${}", i))
                .collect();

            let query = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table_name,
                columns.join(", "),
                placeholders.join(", ")
            );

            // Prépare les valeurs pour la requête
            let mut query_builder = sqlx::query(&query);
            
            for col in &columns {
                let value = &json_data[col];
                
                // Ajoute chaque valeur au query builder selon son type
                match value {
                    serde_json::Value::Null => {
                        query_builder = query_builder.bind::<Option<String>>(None);
                    },
                    serde_json::Value::Bool(b) => {
                        query_builder = query_builder.bind(b);
                    },
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            query_builder = query_builder.bind(i);
                        } else if let Some(f) = n.as_f64() {
                            let decimal_str = format!("{:.3}", f);
                            let decimal = Self::validate_decimal_value(&decimal_str)?;
                            query_builder = query_builder.bind(decimal);
                        } else {
                            return Err(sqlx::Error::Protocol(format!("Unsupported number type for column {}", col)));
                        }
                    },
                    serde_json::Value::String(s) => {
                        if let Ok(decimal) = Self::validate_decimal_value(s) {
                            query_builder = query_builder.bind(decimal);
                        } else {
                            query_builder = query_builder.bind(s.clone());
                        }
                    },
                    serde_json::Value::Array(arr) => {
                        if arr.iter().all(|v| v.is_string()) {
                            let strings: Vec<String> = arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect();
                            query_builder = query_builder.bind(strings);
                        } else {
                            let json_value = Self::validate_json_value(value)?;
                            query_builder = query_builder.bind(json_value);
                        }
                    },
                    _ => {
                        // Pour les objets JSON, on les valide et on les passe directement
                        let json_value = Self::validate_json_value(value)?;
                        query_builder = query_builder.bind(json_value);
                    }
                }
            }

            // Exécute la requête
            query_builder.execute(&mut *tx).await?;
        }

        // Commit la transaction
        tx.commit().await?;

        info!("Successfully submitted {} fixtures to table {}", fixture_len, table_name);
        Ok(())
    }

    /// Nettoie les fixtures d'une table
    pub async fn cleanup_fixtures(&self, table_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table_name))
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

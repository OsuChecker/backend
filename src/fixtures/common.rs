use sqlx::{Pool, Postgres};
use tracing::{info, warn};

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
        info!("Checking if migrations are up to date...");
        
        // Vérifie si la table _sqlx_migrations existe
        let migrations_exist = sqlx::query!(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_name = '_sqlx_migrations'
            ) as exists"
        )
        .fetch_one(&self.pool)
        .await?
        .exists
        .unwrap_or(false);

        if !migrations_exist {
            warn!("No migrations found. Database may not be properly initialized.");
        }

        Ok(())
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
                            query_builder = query_builder.bind(f);
                        } else {
                            return Err(sqlx::Error::Protocol(format!("Unsupported number type for column {}", col)));
                        }
                    },
                    serde_json::Value::String(s) => {
                        query_builder = query_builder.bind(s.clone());
                    },
                    _ => {
                        // Pour les tableaux et objets, on les sérialise en JSON
                        let json_string = match serde_json::to_string(value) {
                            Ok(s) => s,
                            Err(e) => return Err(sqlx::Error::Protocol(format!("JSON serialization error: {}", e))),
                        };
                        query_builder = query_builder.bind(json_string);
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
        info!("Cleaning up fixtures from table {}", table_name);
        
        sqlx::query(&format!("DELETE FROM {}", table_name))
            .execute(&self.pool)
            .await?;

        info!("Successfully cleaned up fixtures from table {}", table_name);
        Ok(())
    }
}

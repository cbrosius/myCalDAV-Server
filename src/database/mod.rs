use sqlx::{sqlite::SqlitePool, Executor};
use std::fs;
use tracing::info;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use uuid::Uuid;

pub async fn initialize_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let migration_dir = "./src/migrations";
    
    if let Ok(entries) = fs::read_dir(migration_dir) {
        let mut migrations: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let path = e.path();
                if path.is_file() && path.extension().map(|s| s == "sql").unwrap_or(false) {
                    path.file_name()
                        .and_then(|name| name.to_str().map(|s| s.to_string()))
                } else {
                    None
                }
            })
            .collect();
        
        migrations.sort();
        
        for migration in migrations {
            let path = format!("{}/{}", migration_dir, migration);
            if let Ok(content) = fs::read_to_string(&path) {
                info!("Running migration: {}", migration);
                pool.execute(content.as_str()).await?;
            }
        }
    }
    
    // Create default user if not exists
    create_default_user(pool).await?;
    
    Ok(())
}

async fn create_default_user(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if default user exists
    let existing_user: Option<(String,)> = sqlx::query_as("SELECT email FROM users WHERE email = ?")
        .bind("test@test.com")
        .fetch_optional(pool)
        .await?;
    
    if existing_user.is_none() {
        info!("Creating default user: test@test.com");
        let password_hash = hash("password123", DEFAULT_COST).expect("Failed to hash password");
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO users (id, name, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind("Test User")
        .bind("test@test.com")
        .bind(&password_hash)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        
        info!("Default user created successfully. Email: test@test.com, Password: password123");
    }
    
    Ok(())
}

#[allow(dead_code)]
pub async fn check_database_schema(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let tables_exist = sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table'")
        .fetch_all(pool)
        .await?;
    
    Ok(!tables_exist.is_empty())
}
use sqlx::{sqlite::SqlitePool, Executor};
use std::fs;
use tracing::info;

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
    
    Ok(())
}

pub async fn check_database_schema(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let tables_exist = sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table'")
        .fetch_all(pool)
        .await?;
    
    Ok(!tables_exist.is_empty())
}
// This module is kept for potential future state extensions
// Currently, the CalendarService is used directly as the application state

use sqlx::sqlite::SqlitePool;

/// Application state container
pub struct AppState {
    pub db_pool: SqlitePool,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = SqlitePool::connect(database_url).await?;
        Ok(Self { db_pool })
    }
}

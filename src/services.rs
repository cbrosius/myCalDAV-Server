use sqlx::sqlite::SqlitePool;
use chrono::Utc;
use uuid::Uuid;
use crate::models::*;
use crate::error::AppError;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Clone)]
pub struct CalendarService {
    pool: SqlitePool,
    jwt_secret: String,
}

impl CalendarService {
    pub fn new(pool: SqlitePool) -> Self {
        CalendarService { 
            pool,
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
        }
    }

    pub fn get_jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }

    // User operations
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, AppError> {
        let password_hash = hash(new_user.password, DEFAULT_COST)?;
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at) 
            VALUES (?, ?, ?, ?, ?)
            RETURNING id, email, password_hash, created_at, updated_at
            "#
        )
        .bind(id.to_string())
        .bind(new_user.email)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(&self, id: Uuid, email: Option<String>, password: Option<String>) -> Result<User, AppError> {
        let now = Utc::now();
        
        if let Some(new_email) = email {
            sqlx::query("UPDATE users SET email = ?, updated_at = ? WHERE id = ?")
                .bind(new_email)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(new_password) = password {
            let password_hash = hash(new_password, DEFAULT_COST)?;
            sqlx::query("UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?")
                .bind(password_hash)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        self.get_user_by_id(id).await?.ok_or(AppError::NotFoundError("User not found".to_string()))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Calendar operations
    pub async fn get_calendars_by_user_id(&self, user_id: Uuid) -> Result<Vec<Calendar>, AppError> {
        let calendars = sqlx::query_as::<_, Calendar>(
            "SELECT id, user_id, name, description, color, is_public, created_at, updated_at FROM calendars WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(calendars)
    }

    pub async fn get_calendar_by_id(&self, id: Uuid) -> Result<Option<Calendar>, AppError> {
        let calendar = sqlx::query_as::<_, Calendar>(
            "SELECT id, user_id, name, description, color, is_public, created_at, updated_at FROM calendars WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(calendar)
    }

    pub async fn create_calendar(&self, user_id: Uuid, new_calendar: NewCalendar) -> Result<Calendar, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        let calendar = sqlx::query_as::<_, Calendar>(
            r#"
            INSERT INTO calendars (id, user_id, name, description, color, is_public, created_at, updated_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id, user_id, name, description, color, is_public, created_at, updated_at
            "#
        )
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(new_calendar.name)
        .bind(new_calendar.description)
        .bind(new_calendar.color)
        .bind(new_calendar.is_public)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(calendar)
    }

    pub async fn update_calendar(&self, id: Uuid, updates: UpdateCalendar) -> Result<Calendar, AppError> {
        let now = Utc::now();
        
        if let Some(name) = updates.name {
            sqlx::query("UPDATE calendars SET name = ?, updated_at = ? WHERE id = ?")
                .bind(name)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(description) = updates.description {
            sqlx::query("UPDATE calendars SET description = ?, updated_at = ? WHERE id = ?")
                .bind(description)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(color) = updates.color {
            sqlx::query("UPDATE calendars SET color = ?, updated_at = ? WHERE id = ?")
                .bind(color)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(is_public) = updates.is_public {
            sqlx::query("UPDATE calendars SET is_public = ?, updated_at = ? WHERE id = ?")
                .bind(is_public)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        self.get_calendar_by_id(id).await?.ok_or(AppError::NotFoundError("Calendar not found".to_string()))
    }

    pub async fn delete_calendar(&self, id: Uuid) -> Result<(), AppError> {
        // First delete all events in this calendar
        sqlx::query("DELETE FROM events WHERE calendar_id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        
        // Then delete all shares for this calendar
        sqlx::query("DELETE FROM shares WHERE calendar_id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        
        // Finally delete the calendar
        sqlx::query("DELETE FROM calendars WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    // Event operations
    pub async fn get_event_by_id(&self, id: Uuid) -> Result<Option<Event>, AppError> {
        let event = sqlx::query_as::<_, Event>(
            "SELECT id, calendar_id, title, description, location, start_time, end_time, is_all_day, created_at, updated_at FROM events WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(event)
    }

    pub async fn get_events_by_calendar_id(&self, calendar_id: Uuid) -> Result<Vec<Event>, AppError> {
        let events = sqlx::query_as::<_, Event>(
            "SELECT id, calendar_id, title, description, location, start_time, end_time, is_all_day, created_at, updated_at FROM events WHERE calendar_id = ?"
        )
        .bind(calendar_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(events)
    }

    pub async fn create_event(&self, calendar_id: Uuid, new_event: NewEvent) -> Result<Event, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        let event = sqlx::query_as::<_, Event>(
            r#"
            INSERT INTO events (id, calendar_id, title, description, location, start_time, end_time, is_all_day, created_at, updated_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id, calendar_id, title, description, location, start_time, end_time, is_all_day, created_at, updated_at
            "#
        )
        .bind(id.to_string())
        .bind(calendar_id.to_string())
        .bind(new_event.title)
        .bind(new_event.description)
        .bind(new_event.location)
        .bind(new_event.start_time)
        .bind(new_event.end_time)
        .bind(new_event.is_all_day)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(event)
    }

    pub async fn update_event(&self, id: Uuid, updates: UpdateEvent) -> Result<Event, AppError> {
        let now = Utc::now();
        
        if let Some(title) = updates.title {
            sqlx::query("UPDATE events SET title = ?, updated_at = ? WHERE id = ?")
                .bind(title)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(description) = updates.description {
            sqlx::query("UPDATE events SET description = ?, updated_at = ? WHERE id = ?")
                .bind(description)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(location) = updates.location {
            sqlx::query("UPDATE events SET location = ?, updated_at = ? WHERE id = ?")
                .bind(location)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(start_time) = updates.start_time {
            sqlx::query("UPDATE events SET start_time = ?, updated_at = ? WHERE id = ?")
                .bind(start_time)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(end_time) = updates.end_time {
            sqlx::query("UPDATE events SET end_time = ?, updated_at = ? WHERE id = ?")
                .bind(end_time)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        if let Some(is_all_day) = updates.is_all_day {
            sqlx::query("UPDATE events SET is_all_day = ?, updated_at = ? WHERE id = ?")
                .bind(is_all_day)
                .bind(now)
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;
        }
        
        self.get_event_by_id(id).await?.ok_or(AppError::NotFoundError("Event not found".to_string()))
    }

    pub async fn delete_event(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM events WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Share operations
    pub async fn get_shares_by_calendar_id(&self, calendar_id: Uuid) -> Result<Vec<Share>, AppError> {
        let shares = sqlx::query_as::<_, Share>(
            "SELECT id, calendar_id, user_id, shared_with_user_id, permission_level, created_at FROM shares WHERE calendar_id = ?"
        )
        .bind(calendar_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(shares)
    }

    pub async fn create_share(&self, calendar_id: Uuid, user_id: Uuid, new_share: NewShare) -> Result<Share, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let permission_str = match new_share.permission_level {
            PermissionLevel::Read => "read",
            PermissionLevel::Write => "write",
            PermissionLevel::Admin => "admin",
        };
        
        let share = sqlx::query_as::<_, Share>(
            r#"
            INSERT INTO shares (id, calendar_id, user_id, shared_with_user_id, permission_level, created_at) 
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id, calendar_id, user_id, shared_with_user_id, permission_level, created_at
            "#
        )
        .bind(id.to_string())
        .bind(calendar_id.to_string())
        .bind(user_id.to_string())
        .bind(new_share.shared_with_user_id.to_string())
        .bind(permission_str)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(share)
    }

    pub async fn delete_share(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM shares WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL DEFAULT '',
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create calendars table
CREATE TABLE IF NOT EXISTS calendars (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    color TEXT,
    is_public INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Create events table
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    location TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    is_all_day INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (calendar_id) REFERENCES calendars (id) ON DELETE CASCADE
);

-- Create shares table
CREATE TABLE IF NOT EXISTS shares (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    shared_with_user_id TEXT,
    shared_with_email TEXT,
    permission_level TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (calendar_id) REFERENCES calendars (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_calendars_user_id ON calendars (user_id);
CREATE INDEX IF NOT EXISTS idx_events_calendar_id ON events (calendar_id);
CREATE INDEX IF NOT EXISTS idx_shares_calendar_id ON shares (calendar_id);
CREATE INDEX IF NOT EXISTS idx_shares_user_id ON shares (user_id);
CREATE INDEX IF NOT EXISTS idx_shares_shared_with_user_id ON shares (shared_with_user_id);

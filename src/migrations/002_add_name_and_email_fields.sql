-- Add name column to users table
ALTER TABLE users ADD COLUMN name TEXT;

-- Update existing users to have a default name from email
UPDATE users SET name = substr(email, 1, instr(email, '@') - 1) WHERE name IS NULL;

-- Add shared_with_email column to shares table
ALTER TABLE shares ADD COLUMN shared_with_email TEXT;

-- Make shared_with_user_id nullable
-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
CREATE TABLE shares_new (
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

-- Copy data from old table
INSERT INTO shares_new (id, calendar_id, user_id, shared_with_user_id, permission_level, created_at)
SELECT id, calendar_id, user_id, shared_with_user_id, permission_level, created_at FROM shares;

-- Drop old table and rename new one
DROP TABLE shares;
ALTER TABLE shares_new RENAME TO shares;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS idx_shares_calendar_id ON shares (calendar_id);
CREATE INDEX IF NOT EXISTS idx_shares_user_id ON shares (user_id);

-- Add role column to users table
ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'user';

-- Create index for role lookups
CREATE INDEX IF NOT EXISTS idx_users_role ON users (role);

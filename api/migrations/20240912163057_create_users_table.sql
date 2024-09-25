-- Create the UUID extension if it does not exist
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the user_role ENUM type
CREATE TYPE user_role AS ENUM (
    'Reviewer',
    'Admin'
);

-- Create the users table
CREATE TABLE IF NOT EXISTS users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name TEXT NOT NULL, -- Full name
    email TEXT NOT NULL UNIQUE, -- Brockport email
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'Reviewer',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add constraints
ALTER TABLE users
    ADD CONSTRAINT check_name_length CHECK (length(name) > 0 AND length(name) <= 128),
    ADD CONSTRAINT check_email_length CHECK (length(email) > 0 AND length(email) <= 128),
    ADD CONSTRAINT check_role CHECK (role IN ('Reviewer', 'Admin'));

-- Create or replace the update_user_timestamp function
CREATE OR REPLACE FUNCTION update_user_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger to update the timestamp before each update on the users table
CREATE TRIGGER update_user_before_update
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_user_timestamp();

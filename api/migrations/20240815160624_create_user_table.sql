-- Add migration script here
CREATE EXTENSION "uuid-ossp";

CREATE TYPE user_role AS ENUM (
    'Reviewer',
    'Admin'
);

CREATE TABLE "user" (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name TEXT NOT NULL, -- Full name
    email TEXT NOT NULL UNIQUE, -- Brockport email
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'Reviewer',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Constraints
ALTER TABLE "user"
    ADD CONSTRAINT check_name_length CHECK (length(name) > 0 AND length(name) <= 256),
    ADD CONSTRAINT check_email_length CHECK (length(email) > 0 AND length(email) <= 256),
    ADD CONSTRAINT check_role CHECK (role IN ('Reviewer', 'Admin'));
    
-- Ensure updated_at is always updated on modification
CREATE OR REPLACE FUNCTION update_user_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Runs function on every update query on "user"
CREATE TRIGGER update_user_before_update
    BEFORE UPDATE ON "user"
    FOR EACH ROW
    EXECUTE FUNCTION update_user_timestamp();

CREATE TABLE requester (
    request_id INTEGER PRIMARY KEY, -- From TeamDynamix ticket
    name TEXT NOT NULL, -- Full name
    email TEXT NOT NULL, -- Brockport email
    department TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Constraints
ALTER TABLE requester
    ADD CONSTRAINT check_name_length CHECK (length(name) > 0 AND length(name) <= 50),
    ADD CONSTRAINT check_email_length CHECK (length(email) > 0 AND length(email) <= 50),
    ADD CONSTRAINT check_department_length CHECK (length(department) > 0 AND length(department) <= 50);

-- Ensure updated_at is always updated on modification
CREATE OR REPLACE FUNCTION update_requester_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Runs function on every update query on requester
CREATE TRIGGER update_requester_before_update
    BEFORE UPDATE ON requester
    FOR EACH ROW
    EXECUTE FUNCTION update_requester_timestamp();

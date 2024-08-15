-- Add migration script here
CREATE TABLE software (
    name TEXT PRIMARY KEY,
    version NUMERIC NOT NULL, -- Version requested
    description TEXT NOT NULL,
    developer TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Constraints
ALTER TABLE software
    ADD CONSTRAINT check_name_length CHECK (length(name) > 0 AND length(name) <= 50),
    ADD CONSTRAINT check_description_length CHECK (length(description) > 0 AND length(description) <= 255),
    ADD CONSTRAINT check_developer_length CHECK (length(developer) > 0 AND length(developer) <= 50);

-- Ensure updated_at is always updated on modification
CREATE OR REPLACE FUNCTION update_software_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Runs function on every update query on software
CREATE TRIGGER update_software_before_update
    BEFORE UPDATE ON software
    FOR EACH ROW
    EXECUTE FUNCTION update_software_timestamp();

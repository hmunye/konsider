-- Create the software table
CREATE TABLE IF NOT EXISTS software (
    name TEXT PRIMARY KEY,
    software_version NUMERIC NOT NULL, -- Version requested
    description TEXT NOT NULL,
    developer TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add constraints
ALTER TABLE software
    ADD CONSTRAINT check_name_length CHECK (length(name) > 0 AND length(name) <= 128),
    ADD CONSTRAINT check_software_version CHECK (software_version > 0.0),
    ADD CONSTRAINT check_description_length CHECK (length(description) > 0 AND length(description) <= 128),
    ADD CONSTRAINT check_developer_length CHECK (length(developer) > 0 AND length(developer) <= 128);

-- Ensure updated_at is always updated on modification
CREATE OR REPLACE FUNCTION update_software_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger to update the timestamp before each update on the software table
CREATE TRIGGER update_software_before_update
    BEFORE UPDATE ON software
    FOR EACH ROW
    EXECUTE FUNCTION update_software_timestamp();

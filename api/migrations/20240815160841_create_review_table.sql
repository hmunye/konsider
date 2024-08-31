-- Add migration script here
CREATE TYPE review_status AS ENUM (
    'UnderReview',
    'Complete'
);

CREATE TABLE review (
    software_name TEXT NOT NULL,
    request_id INTEGER NOT NULL,
    reviewer_email TEXT NOT NULL,
    is_supported BOOLEAN NOT NULL, -- Is supported by the developer
    is_current_version BOOLEAN NOT NULL,
    is_reputation_good BOOLEAN NOT NULL, -- Developer reputation
    is_installation_from_developer BOOLEAN NOT NULL,
    is_local_admin_required BOOLEAN NOT NULL,
    is_connected_to_brockport_cloud BOOLEAN NOT NULL,
    is_connected_to_cloud_services_or_client BOOLEAN NOT NULL,
    is_security_or_optimization_software BOOLEAN NOT NULL,
    is_supported_by_current_os BOOLEAN NOT NULL,
    is_exported BOOLEAN NOT NULL, -- Has the review been exported
    notes TEXT,
    status review_status NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (software_name, request_id, reviewer_email),
    FOREIGN KEY (software_name) REFERENCES software(name) ON DELETE RESTRICT,
    FOREIGN KEY (request_id) REFERENCES requester(request_id) ON DELETE RESTRICT,
    FOREIGN KEY (reviewer_email) REFERENCES users(email) ON DELETE RESTRICT
);

-- Constraints
ALTER TABLE review
    ADD CONSTRAINT check_status CHECK (status IN ('UnderReview', 'Complete'));

-- Ensure updated_at is always updated on modification
CREATE OR REPLACE FUNCTION update_review_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Runs function on every update query on review
CREATE TRIGGER update_review_before_update
    BEFORE UPDATE ON review
    FOR EACH ROW
    EXECUTE FUNCTION update_review_timestamp();

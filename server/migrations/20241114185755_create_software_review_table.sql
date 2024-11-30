CREATE TYPE review_options AS ENUM ('TRUE', 'FALSE', 'NOT_SURE');

CREATE TABLE software_review (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    software_request_id UUID NOT NULL UNIQUE REFERENCES software_request(id) ON DELETE RESTRICT, -- Foreign key to software request. Will error if trying to delete the software request being referenced. One-to-one relationship enforced through UNIQUE
    reviewer_id UUID NOT NULL REFERENCES user_account(id) ON DELETE RESTRICT, -- Foreign key to user account. Will error if trying to delete the user account being referenced
    is_supported review_options NOT NULL, -- Is the software supported by the developer?
    is_current_version review_options NOT NULL, -- Is it the current version of the software?
    is_reputation_good review_options NOT NULL, -- Does the developer have a good reputation?
    is_installation_from_developer review_options NOT NULL, -- Was the software installation obtained from the developer?
    is_local_admin_required review_options NOT NULL, -- Does the software require local admin privileges?
    is_connected_to_brockport_cloud review_options NOT NULL, -- Is the software connected to the Brockport cloud?
    is_connected_to_cloud_services_or_client review_options NOT NULL, -- Is the software connected to cloud services or clients?
    is_security_or_optimization_software review_options NOT NULL, -- Is this security or optimization software?
    is_supported_by_current_os review_options NOT NULL, -- Is the software supported by the current OS?
    exported BOOLEAN NOT NULL DEFAULT FALSE, -- Has the review been exported?
    review_notes VARCHAR(255) NOT NULL CHECK (length(review_notes) > 0), -- Additional notes for the software review
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you know this record has never been updated
    CONSTRAINT unique_software_review UNIQUE (software_request_id, reviewer_id) -- Ensure each review is unique per software request and reviewer pair
);

CREATE TABLE software_review (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    software_request_id UUID NOT NULL UNIQUE REFERENCES software_request(id) ON DELETE RESTRICT, -- Foreign key to software request. Will error if trying to delete the software request being referenced, One-to-one relationship enforced through UNIQUE
    reviewer_id UUID NOT NULL REFERENCES user_account(id) ON DELETE RESTRICT, -- Foreign key to user account. Will error if trying to delete the user account being referenced
    exported BOOLEAN DEFAULT FALSE, -- Has the review been exported?
    review_notes VARCHAR(255) DEFAULT 'NOT PROVIDED', -- Additional notes for the software review
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you known this record has never been updated
    CONSTRAINT unique_software_review UNIQUE (software_request_id, reviewer_id) -- Ensure each review is unique per software request and reviewer pair
);

CREATE TYPE review_options AS ENUM ('TRUE', 'FALSE', 'NOT_SURE');

CREATE TABLE software_review_response (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    software_review_id UUID NOT NULL REFERENCES software_review(id) ON DELETE RESTRICT, -- Foreign key to software review. Will error if trying to delete the software review being referenced
    review_factor_id UUID NOT NULL REFERENCES review_factor(id) ON DELETE RESTRICT, -- Foreign key to review factor. Will error if trying to delete the review factor being referenced
    response review_options NOT NULL, -- Answer to the specific review factor. ENUM type ensures response can only be set to 'TRUE', 'FALSE', or 'NOT SURE'
    CONSTRAINT unique_review_factor_per_review UNIQUE (software_review_id, review_factor_id) -- Ensure each software review response is unique per software review and review factor
);

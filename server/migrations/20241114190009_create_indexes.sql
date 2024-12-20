-- user account
CREATE INDEX idx_user_account_role ON user_account(role);

-- software review
CREATE INDEX idx_software_review_software_request_id ON software_review(software_request_id);
CREATE INDEX idx_software_review_reviewer_id ON software_review(reviewer_id);

-- user token
CREATE INDEX idx_user_token_revoked ON user_token(revoked);

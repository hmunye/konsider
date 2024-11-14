-- Generic trigger function for updated_at
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- user account
CREATE TRIGGER update_user_account_timestamp_before_update
    BEFORE UPDATE ON user_account
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
    
-- requester
CREATE TRIGGER update_requester_timestamp_before_update
    BEFORE UPDATE ON requester
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
    
-- software
CREATE TRIGGER update_software_timestamp_before_update
    BEFORE UPDATE ON software
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
    
-- software request
CREATE TRIGGER update_software_request_timestamp_before_update
    BEFORE UPDATE ON software_request
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
    
-- software review
CREATE TRIGGER update_software_review_timestamp_before_update
    BEFORE UPDATE ON software_review
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- user token
CREATE TRIGGER update_user_token_timestamp_before_update
    BEFORE UPDATE ON user_token
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

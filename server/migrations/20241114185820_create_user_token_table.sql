CREATE TABLE user_token (
    jti UUID PRIMARY KEY, -- The unique JWT ID for token revocation tracking
    user_id UUID NOT NULL UNIQUE REFERENCES user_account(id) ON DELETE CASCADE, -- Foreign key to user account. Will delete the row if trying to delete the user account being referenced. One-to-one relationship enforced through UNIQUE
    revoked BOOLEAN DEFAULT FALSE, -- Indicates whether the token has been revoked
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() -- If it is the same value as created_at, you known this record has never been updated
);

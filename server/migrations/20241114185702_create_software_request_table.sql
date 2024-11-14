CREATE TABLE software_request (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    td_request_id VARCHAR(8) UNIQUE NOT NULL CHECK (td_request_id ~ '^[0-9]{8}$'), -- TeamDynamix ticket #, ensure it is exactly 8 digits (It is UNIQUE so an INDEX is created automatically)
    software_id UUID NOT NULL UNIQUE REFERENCES software(id) ON DELETE RESTRICT, -- Foreign key to software. Will error if trying to delete the software being referenced. One-to-one relationship enforced through UNIQUE
    requester_id UUID NOT NULL REFERENCES requester(id) ON DELETE RESTRICT, -- Foreign key to requester. Will error if trying to delete the requester being referenced
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() -- If it is the same value as created_at, you known this record has never been updated
);

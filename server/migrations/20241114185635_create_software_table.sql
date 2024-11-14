CREATE TABLE software (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    software_name VARCHAR(100) NOT NULL UNIQUE CHECK (length(software_name) > 0), -- Name of software (It is UNIQUE so an INDEX is created automatically)
    software_version VARCHAR(5) NOT NULL CHECK (software_version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'), -- Version of software. Enforces x.y.z format
    developer_name VARCHAR(100) NOT NULL CHECK (length(developer_name) > 0), -- Developer/Vendor of the software
    description VARCHAR(255) NOT NULL CHECK (length(description) > 0), -- Description of the software
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() -- If it is the same value as created_at, you known this record has never been updated
);

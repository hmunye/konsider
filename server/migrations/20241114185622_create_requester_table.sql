CREATE TABLE requester (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL CHECK (length(name) > 0), -- Full name of requester
    email VARCHAR(255) NOT NULL UNIQUE CHECK(length(email) > 0), -- Brockport email (It is UNIQUE so an INDEX is created automatically)
    department VARCHAR(100) NOT NULL CHECK (length(department) > 0), -- Department of requester
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() -- If it is the same value as created_at, you known this record has never been updated
);

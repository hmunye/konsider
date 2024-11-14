CREATE TYPE user_role AS ENUM ('REVIEWER', 'ADMIN');

CREATE TABLE user_account (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL CHECK (length(name) > 0), -- Full name
    email VARCHAR(255) NOT NULL UNIQUE CHECK(length(email) > 0), -- Brockport email (It is UNIQUE so an INDEX is created automatically)
    password_hash TEXT NOT NULL,
    role user_role DEFAULT 'REVIEWER', -- ENUM type ensures role can only be set to 'REVIEWER' or 'ADMIN'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() -- If it is the same value as created_at, you known this record has never been updated
);

# K6R Database Design (PostgreSQL)

## Goals

Use **3rd Normal Form** (**3NF**) ✅

> **Database Normalization** is the process of organizing a relational database to reduce redundancy and improve data integrity

> Starting from no normalization, to achieve **1NF**, remove repeating groups and ensure atomic values; to reach **2NF**, eliminate partial dependencies (for composite keys); and to reach **3NF**, remove transitive dependencies

Use **Constraints** ✅

Define **Relationships** ✅

Use **Indexing** ✅

> Single-column indexing, composite indexes, etc. 

~~Use~~ ~~**Row Level Security**~~

> ~~Control access to data at the row level, ensuring that users only access the data they are authorized to see~~

Use **Triggers** ✅

> Automatically enforce data consistency, such as updating timestamps, etc.

Simple Strategy for **Database Backup** and **Recovery** ✅

> Automated Backups, Point-in-Time Recovery, etc.

Use **SSL** for Database Connections ✅    

~~Simplicity with~~ ~~**Views**~~

> ~~Use views to simplify access to frequently used or complex queries~~

 ~~**Archive**~~ ~~deleted data for historical and auditing purposes~~

---
## Extensions

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Provides UUID type and uuid_generate_v4() function used in each table 
```
---

## Schema
### 1. User Account:

```sql
CREATE TYPE user_role AS ENUM ('REVIEWER', 'ADMIN');

CREATE TABLE user_account (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL CHECK (length(name) > 0), -- Full name
    email VARCHAR(255) NOT NULL UNIQUE CHECK(length(email) > 0), -- Brockport email (It is UNIQUE so an INDEX is created automatically)
    password_hash BYTEA NOT NULL,
    role user_role DEFAULT 'REVIEWER', -- ENUM type ensures role can only be set to 'REVIEWER' or 'ADMIN'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you known this record has never been updated
    version INT DEFAULT 1
);
```
### 2. Requester:

```sql
CREATE TABLE requester (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL CHECK (length(name) > 0), -- Full name of requester
    email VARCHAR(255) NOT NULL UNIQUE CHECK(length(email) > 0), -- Brockport email (It is UNIQUE so an INDEX is created automatically)
    department VARCHAR(100) NOT NULL CHECK (length(department) > 0), -- Department of requester
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you known this record has never been updated
    version INT DEFAULT 1
);
```
### 3. Software:

```sql
CREATE TABLE software (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    software_name VARCHAR(100) NOT NULL UNIQUE CHECK (length(software_name) > 0), -- Name of software (It is UNIQUE so an INDEX is created automatically)
    software_version VARCHAR(12) NOT NULL CHECK (software_version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'), -- Version of software. Enforces x.y.z format
    developer_name VARCHAR(100) NOT NULL CHECK (length(developer_name) > 0), -- Developer/Vendor of the software
    description VARCHAR(255) NOT NULL CHECK (length(description) > 0), -- Description of the software
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you known this record has never been updated
    version INT DEFAULT 1
);
```
### 4. Software Request:

```sql
CREATE TABLE software_request (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    td_request_id VARCHAR(8) UNIQUE NOT NULL CHECK (td_request_id ~ '^[0-9]{8}$'), -- TeamDynamix ticket #, ensure it is exactly 8 digits (It is UNIQUE so an INDEX is created automatically)
    software_id UUID NOT NULL UNIQUE REFERENCES software(id) ON DELETE RESTRICT, -- Foreign key to software. Will error if trying to delete the software being referenced. One-to-one relationship enforced through UNIQUE
    requester_id UUID NOT NULL REFERENCES requester(id) ON DELETE RESTRICT, -- Foreign key to requester. Will error if trying to delete the requester being referenced
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you known this record has never been updated
    version INT DEFAULT 1
);
```
### 5. Software Review:

```sql
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
    exported BOOLEAN DEFAULT FALSE, -- Has the review been exported?
    review_notes VARCHAR(255) DEFAULT 'NOT PROVIDED', -- Additional notes for the software review
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- If it is the same value as created_at, you know this record has never been updated
    CONSTRAINT unique_software_review UNIQUE (software_request_id, reviewer_id) -- Ensure each review is unique per software request and reviewer pair
);
```
### 6. User Token:

```sql
CREATE TABLE user_token (
    jti UUID PRIMARY KEY, -- The unique JWT ID for token revocation tracking
    user_id UUID NOT NULL UNIQUE REFERENCES user_account(id) ON DELETE CASCADE, -- Foreign key to user account. Will delete the row if trying to delete the user account being referenced. One-to-one relationship enforced through UNIQUE
    revoked BOOLEAN DEFAULT FALSE, -- Indicates whether the token has been revoked
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() -- If it is the same value as created_at, you known this record has never been updated
);
```
---
## Triggers

```sql
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
```
---
## Indexes

```sql
-- user account
CREATE INDEX idx_user_account_role ON user_account(role);

-- software review
CREATE INDEX idx_software_review_software_request_id ON software_review(software_request_id);
CREATE INDEX idx_software_review_reviewer_id ON software_review(reviewer_id);

-- user token
CREATE INDEX idx_user_token_revoked ON user_token(revoked);
```
---
## Relationships

### User Account to Software Review:

**Type**: One-to-Many

> Each user can review multiple software requests, but each software review is assigned to only one user

### Requester to Software Request:

**Type**: One-to-Many

> Each requester can make multiple software requests, but each software request is linked to only one requester

### Software to Software Request:

**Type**: One-to-One

> Each software item can be requested only once in a software request, and each software request is linked to only one software item. This is to avoid duplicate requests for software

### Software Request to Software Review:

**Type**: One-to-One

> Each software request can have only one associated review, and each software review is linked to only one software request. This is to avoid multiple reviews for a single request

### User Account to User Token:

Type: One-to-One

> Each user can have only one token associated to their account at a time and each token can only be associated to one user

---

## Simple Database Backup and Recovery

> Assuming PostgreSQL instances are running within **Docker** containers


Using PostgreSQL's built-in **pg_dump** and **pg_dumpall** CLI tools, we can easily back up individual or all databases

### Backup using pg_dump:

```bash
DATE=$(date +%Y%m%d-%H%M%S) # Sets DATE environment variable

docker exec [container_id] pg_dump -U [pg_user] -d [db_name] -Fc | gzip -9c > "$PWD/db-backup-$DATE.dump.gz"
# ^ Dumps a specific PostgreSQL database into a custom format file. Using gzip, the file is compressed to save storage space
```

> `pg_dump`: This utility dumps a specified PostgreSQL database, rather than all databases in the container

> Used the `-Fc` option to specify that the backup should be in **custom format **for pg_dump

> Using the `-9c` option with gzip uses the maximum level of compression and redirects its output to stdout so it can be appended to the file

> Replace `[container_id]`, `[pg_user]`, and `[db_name]` with values matching your setup

### Backup using pg_dumpall:

```bash
DATE=$(date +%Y%m%d-%H%M%S) # Sets DATE environment variable

docker exec [container_id] pg_dumpall -U [pg_user] | gzip -9c > "$PWD/db-backup-$DATE.sql.gz"
# ^ Dumps all PostgreSQL databases into a plain text SQL file. Using gzip, the file is compressed to save storage space
```

> `pg_dumpall`: This utility dumps all PostgreSQL databases in your container

> Does not support the  `-Fc` option, instead will output plain text SQL

> Replace `[container_id]` and `[pg_user]` with values matching your setup

### Restoring from Backup (Where pg_dump used to backup):

``` bash
# Step 1: Decompress the backup file
gunzip "$PWD/db-backup-YYYYMMDD-HHMMSS.dump.gz"

# Step 2: Copy the decompressed backup file into the container
docker cp "$PWD/db-backup-YYYYMMDD-HHMMSS.dump" [container_id]:/db-backup.dump

# Step 3: Access the shell inside the container
docker exec -it [container_id] /bin/bash

# Step 4: Create the database (if it doesn't exist)
psql -U [pg_user] -c "CREATE DATABASE [db_name];"

# Step 5: Run pg_restore from within the container
pg_restore -U [pg_user] -d [db_name] --no-owner -1 -Fc /db-backup.dump
```

> Replace `[container_id]`, `[pg_user]`, and `[db_name]` with values matching your setup

> `pg_restore` restores the database from a dump file inside the container using the specified user and database name, while ensuring no ownership conflicts with `--no-owner` 

> `-1` option for `pg_restore` forces the **entire restore process to be wrapped in a single transaction**. This means that all the restore operations (e.g., creating tables, inserting data) will be treated as one atomic transaction. If any part of the restore fails, **the entire operation will be rolled back**, ensuring that the database is left in a consistent state

### Restoring from Backup (Where pg_dumpall used to backup):

```bash
# Step 1: Decompress the backup file
gunzip "$PWD/db-backup-YYYYMMDD-HHMMSS.sql.gz"

# Step 2: Copy the decompressed backup file into the container
docker cp "$PWD/db-backup-YYYYMMDD-HHMMSS.sql" [container_id]:/db-backup.sql

# Step 3: Access the shell inside the container
docker exec -it [container_id] /bin/bash

# Step 4: Use psql from within the container
psql -U [pg_user] -f /db-backup.sql
```

> Replace `[container_id]` and `[pg_user]` with values matching your setup

---

## Configuring SSL for PostgreSQL

> Assuming PostgreSQL instances are running within **Docker** containers using **Docker Compose**


PostgreSQL has native support for using SSL connections to encrypt client/server communications for increased security

### 1. Create a Self-Signed Certificate:

You can generate a self-signed certificate using OpenSSL. This certificate will be used by PostgreSQL for SSL connections

Command:

```bash
openssl req -new -x509 -days 365 -nodes -text -out server.crt \
-keyout server.key -subj "/CN=dbhost.yourdomain.com"
```

> Replace `[dbhost.yourdomain.com]` with value matching your setup

> `openssl req -new -x509` : Creates a new X.509 certificate (standard format for SSL/TLS certificates)

> `-days 365` : Makes the certificate valid for 365 days

> `-nodes` : Creates the certificate and key without password protection (useful for server use)

> `-out server.crt` : The generated certificate will be saved as `server.crt` 

> `-keyout server.key` : The private key associated with the certificate will be saved as `server.key` 

> `-subj "/CN=dbhost.yourdomain.com"` : Specifies the Common Name (CN), which should be the server's domain or hostname


### 2. Restrict Permissions on the Private Key:

PostgreSQL requires that the private key (`server.key`) is properly protected, so it will reject any key with overly permissive permissions.

Run this command to restrict permissions:

```bash
chmod og-rwx server.key
```
After these steps, you should now have:

- `server.crt`  – Server certificate
- `server.key`  – Server private key

### 3. Configure PostgreSQL to Use SSL:

Modify your `postgresql.conf` to enable SSL by adding the following lines:

```bash
ssl = on
ssl_cert_file = '/etc/ssl/certs/server.crt'
ssl_key_file = '/etc/ssl/certs/server.key'
```

Modify `docker-compose.yml` to mount SSL certificates and postgresql.conf:

```yaml
services:
  postgres:
    image: postgres:alpine
    container_name: prod-postgres
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      # Mount the custom postgresql.conf for SSL setup
      - ./path-to-your-postgres-conf:/etc/postgresql/postgresql.conf
      # Mount the SSL certificates directory making sure it contains server.crt and server.key
      - ./path-to-your-certs:/etc/ssl/certs
    command: ["postgres", "-c", "config_file=/etc/postgresql/postgresql.conf"]
```
---

## Future Improvements and Considerations

If additional review factors are anticipated in the future, you should consider reorganizing the `software_review` table as follows:


### Review Factor:

```sql
-- Current factors:
--  is_supported (Is the software still supported by the developer?)
--  is_current_version (Is the latest version of the software being requested?)
--  is_reputation_good (Does the developer have a good reputation?)
--  is_installation_from_developer (Is the installation package from the developer/vendor?)
--  is_local_admin_required (Is a local administrator required for daily use?)
--  is_connected_to_brockport_cloud (Does the software need to connect to Brockport cloud?)
--  is_connected_to_cloud_services_or_client (Does the software need to connect to other cloud services or is a client for a cloud service?)
--  is_security_or_optimization_software (Is the software for security or system optimization?)
--  is_supported_by_current_os (Is the software supported by current OS used by devices on campus?)

CREATE TABLE review_factor (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- Factor name, e.g., 'is_supported' (It is UNIQUE so an INDEX is created automatically)
    description VARCHAR(255) NOT NULL, -- Description providing context for each review factor
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```
### Software Review:

```sql
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
```
### Software Review Response:

```sql
CREATE TYPE review_options AS ENUM ('TRUE', 'FALSE', 'NOT_SURE');

CREATE TABLE software_review_response (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    software_review_id UUID NOT NULL REFERENCES software_review(id) ON DELETE RESTRICT, -- Foreign key to software review. Will error if trying to delete the software review being referenced
    review_factor_id UUID NOT NULL REFERENCES review_factor(id) ON DELETE RESTRICT, -- Foreign key to review factor. Will error if trying to delete the review factor being referenced
    response review_options NOT NULL, -- Answer to the specific review factor. ENUM type ensures response can only be set to 'TRUE', 'FALSE', or 'NOT SURE'
    CONSTRAINT unique_review_factor_per_review UNIQUE (software_review_id, review_factor_id) -- Ensure each software review response is unique per software review and review factor
);
```

## Relationships

### Software Review to Software Review Response:

**Type**: One-to-Many

> Each software review can have multiple review responses, where each response corresponds to a different review factor. Each software review response is linked to one software review

### Review Factor to Software Review Response:

**Type**: One-to-Many

> Each review factor can be used in multiple software review responses, but each response links to only one factor


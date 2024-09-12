DROP TRIGGER IF EXISTS update_user_before_update ON users;

DROP FUNCTION IF EXISTS update_user_timestamp;

DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS user_role;

DROP EXTENSION IF EXISTS "uuid-ossp";

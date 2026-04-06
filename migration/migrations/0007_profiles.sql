-- =================================================================
-- 0007_profiles.sql
-- =================================================================
CREATE TABLE IF NOT EXISTS user_profiles (
    id           BIGSERIAL    PRIMARY KEY,
    user_id      BIGINT       NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    user_uuid    UUID         NOT NULL,

    display_name VARCHAR(64)  NOT NULL,
    tagline      VARCHAR(100),
    bio          TEXT,
    avatar_url   TEXT,

    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id
    ON user_profiles(user_id);

CREATE TRIGGER trg_user_profiles_updated_at
BEFORE UPDATE ON user_profiles
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_user_profiles_protect_created_at
BEFORE UPDATE ON user_profiles
FOR EACH ROW EXECUTE FUNCTION protect_created_at();
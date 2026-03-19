-- user_profile（1:1）

CREATE TABLE IF NOT EXISTS user_profile (
    user_id BIGINT PRIMARY KEY
        REFERENCES users(id)
        ON DELETE CASCADE,

    display_name VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(512),

    tagline VARCHAR(100),
    bio TEXT,

    locale VARCHAR(50),
    timezone VARCHAR(50),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (tagline IS NULL OR char_length(tagline) <= 100)
);

CREATE TRIGGER trg_user_profile_updated_at
BEFORE UPDATE ON user_profile
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
-- =================================================================
-- 0003_auth_tables.sql
-- =================================================================

-- identity_type enum
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'identity_type') THEN
        CREATE TYPE identity_type AS ENUM (
            'username'
            -- 'email', 'oauth_google', 'oauth_discord', 'oauth_github', 'oauth_microsoft'
        );
    END IF;
END$$;

-- credential_type enum
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'credential_type') THEN
        CREATE TYPE credential_type AS ENUM (
            'password'
            -- 将来: 'totp', 'recovery_code', 'passkey' など
        );
    END IF;
END$$;

-- =================================================================
-- user_identities
-- =================================================================
CREATE TABLE IF NOT EXISTS user_identities (
    id          BIGSERIAL      PRIMARY KEY,
    user_id     BIGINT         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type        identity_type  NOT NULL,
    identifier  VARCHAR(255) NOT NULL,  -- 保存前にアプリ側で正規化すること
    is_primary  BOOLEAN        NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_identity_type_value UNIQUE (type, identifier)
);

CREATE INDEX IF NOT EXISTS idx_user_identities_user_id
ON user_identities(user_id);

CREATE TRIGGER trg_user_identities_updated_at
BEFORE UPDATE ON user_identities
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_user_identities_protect_created_at
BEFORE UPDATE ON user_identities
FOR EACH ROW EXECUTE FUNCTION protect_created_at();

-- =================================================================
-- user_credentials
-- =================================================================
CREATE TABLE IF NOT EXISTS user_credentials (
    id              BIGSERIAL        PRIMARY KEY,
    user_id         BIGINT           NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_type credential_type  NOT NULL DEFAULT 'password',
    secret          TEXT             NOT NULL, -- password_hash / totp_secret / passkey_public_key etc
    created_at      TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_user_credential_type UNIQUE (user_id, credential_type)
);

CREATE INDEX IF NOT EXISTS idx_user_credentials_user_id
ON user_credentials(user_id);

CREATE TRIGGER trg_user_credentials_updated_at
BEFORE UPDATE ON user_credentials
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_user_credentials_protect_created_at
BEFORE UPDATE ON user_credentials
FOR EACH ROW EXECUTE FUNCTION protect_created_at();
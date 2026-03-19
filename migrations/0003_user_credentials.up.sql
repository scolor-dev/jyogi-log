-- user_credentials
-- 認証手段（password / totp / webauthn / oauth等）

CREATE TABLE IF NOT EXISTS user_credentials (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    type VARCHAR(50) NOT NULL,
    -- password / totp / webauthn / oauth_refresh など

    secret_hash TEXT NOT NULL,
    -- パスワード・トークン・鍵など（必ずハッシュ）

    secret_meta JSONB,
    -- 公開鍵・設定情報など

    is_primary BOOLEAN NOT NULL DEFAULT FALSE,

    last_used_at TIMESTAMPTZ,
    verified_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_credentials_user_id
ON user_credentials(user_id);

CREATE INDEX IF NOT EXISTS idx_user_credentials_type
ON user_credentials(type);

CREATE UNIQUE INDEX IF NOT EXISTS ux_user_credentials_primary
ON user_credentials(user_id)
WHERE is_primary = TRUE AND revoked_at IS NULL;

CREATE TRIGGER trg_user_credentials_updated_at
BEFORE UPDATE ON user_credentials
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
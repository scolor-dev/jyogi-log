-- client_tokens
-- M2Mサービス向けトークン管理

CREATE TABLE IF NOT EXISTS client_tokens (
    id BIGSERIAL PRIMARY KEY,

    client_id BIGINT NOT NULL
        REFERENCES oauth_clients(id)
        ON DELETE CASCADE,

    token_hash TEXT NOT NULL,
    -- クライアントトークン（必ずハッシュ）

    scope TEXT,
    -- 付与されたスコープ（スペース区切り）

    last_used_at TIMESTAMPTZ,
    expires_at   TIMESTAMPTZ NOT NULL,
    revoked_at   TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_client_tokens_token_hash
ON client_tokens(token_hash)
WHERE revoked_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_client_tokens_client_id
ON client_tokens(client_id);

CREATE INDEX IF NOT EXISTS idx_client_tokens_expires_at
ON client_tokens(expires_at);

CREATE TRIGGER trg_client_tokens_updated_at
BEFORE UPDATE ON client_tokens
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

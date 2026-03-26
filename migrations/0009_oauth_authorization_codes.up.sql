-- oauth_authorization_codes
-- 認可コード管理（短命・使い捨て）

CREATE TABLE IF NOT EXISTS oauth_authorization_codes (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    client_id BIGINT NOT NULL
        REFERENCES oauth_clients(id)
        ON DELETE CASCADE,

    session_id BIGINT
        REFERENCES sessions(id)
        ON DELETE SET NULL,

    code_hash TEXT NOT NULL,
    -- 認可コード（必ずハッシュ）

    code_challenge        TEXT,
    code_challenge_method VARCHAR(10),
    -- PKCE対応（S256 / plain）

    redirect_uri TEXT NOT NULL,

    scope TEXT,
    -- 要求されたスコープ（スペース区切り）

    expires_at  TIMESTAMPTZ NOT NULL,
    consumed_at TIMESTAMPTZ,
    -- 使用済みフラグ（一度使ったら consumed_at を記録）

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_oauth_authorization_codes_code_hash
ON oauth_authorization_codes(code_hash)
WHERE consumed_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_oauth_authorization_codes_user_id
ON oauth_authorization_codes(user_id);

CREATE INDEX IF NOT EXISTS idx_oauth_authorization_codes_client_id
ON oauth_authorization_codes(client_id);

CREATE INDEX IF NOT EXISTS idx_oauth_authorization_codes_expires_at
ON oauth_authorization_codes(expires_at);

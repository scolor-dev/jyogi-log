-- oauth_clients
-- OAuthクライアント管理

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'oauth_client_type') THEN
        CREATE TYPE oauth_client_type AS ENUM (
            'public',
            'confidential'
        );
    END IF;
END$$;

CREATE TABLE IF NOT EXISTS oauth_clients (
    id BIGSERIAL PRIMARY KEY,

    client_id   VARCHAR(255) NOT NULL UNIQUE,
    client_name VARCHAR(255) NOT NULL,

    client_type oauth_client_type NOT NULL,
    -- public: SPA・モバイルアプリ（client_secretなし）
    -- confidential: サーバーサイドアプリ（client_secretあり）

    client_secret_hash TEXT,
    -- confidential のみ使用（必ずハッシュ）

    revoked_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (
        (client_type = 'public'       AND client_secret_hash IS NULL) OR
        (client_type = 'confidential' AND client_secret_hash IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_oauth_clients_client_id
ON oauth_clients(client_id)
WHERE revoked_at IS NULL;

CREATE TRIGGER trg_oauth_clients_updated_at
BEFORE UPDATE ON oauth_clients
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

-- oauth_client_redirect_uris
-- クライアントごとの許可リダイレクトURI

CREATE TABLE IF NOT EXISTS oauth_client_redirect_uris (
    id BIGSERIAL PRIMARY KEY,

    client_id BIGINT NOT NULL
        REFERENCES oauth_clients(id)
        ON DELETE CASCADE,

    redirect_uri TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_oauth_client_redirect_uris
ON oauth_client_redirect_uris(client_id, redirect_uri);

CREATE INDEX IF NOT EXISTS idx_oauth_client_redirect_uris_client_id
ON oauth_client_redirect_uris(client_id);

-- oauth_scopes
-- スコープ定義

CREATE TABLE IF NOT EXISTS oauth_scopes (
    id BIGSERIAL PRIMARY KEY,

    name        VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- oauth_client_scopes
-- クライアントに許可するスコープの中間テーブル

CREATE TABLE IF NOT EXISTS oauth_client_scopes (
    client_id BIGINT NOT NULL
        REFERENCES oauth_clients(id)
        ON DELETE CASCADE,

    scope_id BIGINT NOT NULL
        REFERENCES oauth_scopes(id)
        ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (client_id, scope_id)
);

CREATE INDEX IF NOT EXISTS idx_oauth_client_scopes_client_id
ON oauth_client_scopes(client_id);

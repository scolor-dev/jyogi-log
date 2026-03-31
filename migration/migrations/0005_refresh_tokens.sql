-- =================================================================
-- 0005_refresh_tokens.sql
-- =================================================================
CREATE TABLE IF NOT EXISTS refresh_tokens (
    id            BIGSERIAL    PRIMARY KEY,
    token_hash    TEXT         NOT NULL UNIQUE,
    session_id    BIGINT       NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    user_id       BIGINT       NOT NULL REFERENCES users(id)    ON DELETE CASCADE,

    -- JWT再生成に必要な情報を非正規化
    user_uuid     UUID         NOT NULL,
    session_uuid  UUID         NOT NULL,

    is_used       BOOLEAN      NOT NULL DEFAULT false,
    expires_at    TIMESTAMPTZ  NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_session_id
    ON refresh_tokens(session_id);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id
    ON refresh_tokens(user_id);

-- 期限切れ・使用済みトークンのクリーンアップ用（部分インデックス）
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_active
    ON refresh_tokens(token_hash)
    WHERE is_used = false AND expires_at > NOW();
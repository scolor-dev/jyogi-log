-- =================================================================
-- 0004_sessions.sql
-- =================================================================
CREATE TABLE IF NOT EXISTS sessions (
    id              BIGSERIAL    PRIMARY KEY,
    session_uuid    UUID         NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    user_id         BIGINT       NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    user_uuid       UUID         NOT NULL,                      -- 非正規化（JOINレス用）
 
    ip_address      INET,                                       -- ログイン時のIP
    user_agent      TEXT,                                       -- ブラウザ/クライアント情報
 
    last_active_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ  NOT NULL,                      -- セッション有効期限
    revoked_at      TIMESTAMPTZ,                                -- 明示的なログアウト
 
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
 
CREATE INDEX IF NOT EXISTS idx_sessions_user_id
    ON sessions(user_id);
 
CREATE INDEX IF NOT EXISTS idx_sessions_session_token
    ON sessions(session_token)
    WHERE revoked_at IS NULL;
 
CREATE TRIGGER trg_sessions_updated_at
BEFORE UPDATE ON sessions
FOR EACH ROW EXECUTE FUNCTION set_updated_at();
 
CREATE TRIGGER trg_sessions_protect_created_at
BEFORE UPDATE ON sessions
FOR EACH ROW EXECUTE FUNCTION protect_created_at();
 
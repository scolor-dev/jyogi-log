-- roles
-- ロール定義（権限はboolカラムで管理）

CREATE TABLE IF NOT EXISTS roles (
    id BIGSERIAL PRIMARY KEY,

    name        VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,

    -- 管理者系
    is_super_admin       BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_users     BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_roles     BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_clients   BOOLEAN NOT NULL DEFAULT FALSE,
    can_manage_scopes    BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_audit_logs  BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_auth_events BOOLEAN NOT NULL DEFAULT FALSE,

    -- 運用系
    can_revoke_tokens   BOOLEAN NOT NULL DEFAULT FALSE,
    can_revoke_sessions BOOLEAN NOT NULL DEFAULT FALSE,
    can_view_users      BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_roles_name
ON roles(name);

CREATE TRIGGER trg_roles_updated_at
BEFORE UPDATE ON roles
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

-- user_roles
-- ユーザーとロールの中間テーブル

CREATE TABLE IF NOT EXISTS user_roles (
    user_id BIGINT NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    role_id BIGINT NOT NULL
        REFERENCES roles(id)
        ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (user_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_user_roles_user_id
ON user_roles(user_id);

CREATE INDEX IF NOT EXISTS idx_user_roles_role_id
ON user_roles(role_id);

-- =================================================================
-- 0006_rbac.sql
-- =================================================================

CREATE TABLE IF NOT EXISTS roles (
    id          BIGSERIAL    PRIMARY KEY,
    name        CITEXT       NOT NULL UNIQUE,
    description TEXT,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trg_roles_updated_at
BEFORE UPDATE ON roles
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER trg_roles_protect_created_at
BEFORE UPDATE ON roles
FOR EACH ROW EXECUTE FUNCTION protect_created_at();

CREATE TABLE IF NOT EXISTS permissions (
    id          BIGSERIAL    PRIMARY KEY,
    resource    VARCHAR(64)  NOT NULL,
    action      VARCHAR(64)  NOT NULL,
    description TEXT,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_permission UNIQUE (resource, action)
);

CREATE TABLE IF NOT EXISTS role_permissions (
    role_id       BIGINT  NOT NULL REFERENCES roles(id)       ON DELETE CASCADE,
    permission_id BIGINT  NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id    BIGINT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id    BIGINT      NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- =================================================================
-- 初期データ
-- =================================================================

INSERT INTO roles (name, description) VALUES
    ('admin',     '全権限'),
    ('user',      '一般ユーザー')
ON CONFLICT DO NOTHING;
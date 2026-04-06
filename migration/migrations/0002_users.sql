-- user_status enum
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_status') THEN
        CREATE TYPE user_status AS ENUM (
            'pending',
            'active',
            'inactive',
            'suspended'
        );
    END IF;
END$$;

-- users
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    status user_status NOT NULL DEFAULT 'pending',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_users_status
ON users(status);


-- users テーブルに updated_at 自動更新トリガーを追加
CREATE TRIGGER trg_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

-- users テーブルに created_at 保護トリガーを追加
CREATE TRIGGER trg_users_protect_created_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION protect_created_at();
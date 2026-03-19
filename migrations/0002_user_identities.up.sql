-- user_identities
CREATE TABLE IF NOT EXISTS user_identities (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    type VARCHAR(50) NOT NULL,

    identifier VARCHAR(255) NOT NULL,
    normalized_identifier VARCHAR(255) NOT NULL,

    is_primary BOOLEAN NOT NULL DEFAULT FALSE,

    last_used_at TIMESTAMPTZ,
    verified_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (char_length(identifier) > 0),
    CHECK (char_length(normalized_identifier) > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_user_identities_active_identifier
ON user_identities(type, normalized_identifier)
WHERE revoked_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_user_identities_user_id
ON user_identities(user_id);

CREATE UNIQUE INDEX IF NOT EXISTS ux_user_identities_primary
ON user_identities(user_id)
WHERE is_primary = TRUE AND revoked_at IS NULL;

CREATE TRIGGER trg_user_identities_updated_at
BEFORE UPDATE ON user_identities
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
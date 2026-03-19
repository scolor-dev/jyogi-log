-- extensions & shared functions
-- downは通常不要（本番でextension削除しないため）

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- 共通: updated_at 自動更新
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
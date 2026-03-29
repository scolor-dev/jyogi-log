-- =================================================================
-- extensions
-- =================================================================
-- down は通常不要（本番で extension を削除しないため）
CREATE EXTENSION IF NOT EXISTS pgcrypto;  -- gen_random_uuid(), ハッシュ
CREATE EXTENSION IF NOT EXISTS pg_trgm;   -- あいまい検索の高速化
CREATE EXTENSION IF NOT EXISTS citext;    -- 大文字小文字を無視する TEXT 型
 
-- =================================================================
-- 共通関数
-- =================================================================
 
-- updated_at 自動更新
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
 
-- created_at を後から変更させない
CREATE OR REPLACE FUNCTION protect_created_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at = OLD.created_at;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

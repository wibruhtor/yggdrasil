-- Add down migration script here
ALTER TABLE IF EXISTS chat_settings DROP CONSTRAINT IF EXISTS chat_settings_ban_word_filter_id_fkey;

-- Add up migration script here
ALTER TABLE IF EXISTS chat_settings
    ADD FOREIGN KEY (ban_word_filter_id)
    REFERENCES public.ban_word_filters (id)
    ON UPDATE CASCADE
    ON DELETE SET NULL;

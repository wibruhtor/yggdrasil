-- Add up migration script here
CREATE TABLE IF NOT EXISTS chat_settings (
  id uuid PRIMARY KEY,
  name varchar NOT NULL,
  chat_type varchar NOT NULL,
  nickname_color integer NOT NULL DEFAULT 3941000703,
  background_color integer NOT NULL DEFAULT 303239167,
  text_color integer NOT NULL DEFAULT 3941000703,
  gradient_only_for_custom_nicknames boolean NOT NULL DEFAULT true,
  margin_top double precision NOT NULL DEFAULT 1,
  margin_right double precision NOT NULL DEFAULT 1,
  margin_bottom double precision NOT NULL DEFAULT 1,
  margin_left double precision NOT NULL DEFAULT 1,
  padding_top double precision NOT NULL DEFAULT 1,
  padding_right double precision NOT NULL DEFAULT 1,
  padding_bottom double precision NOT NULL DEFAULT 1,
  padding_left double precision NOT NULL DEFAULT 1,
  border_top_left_radius double precision NOT NULL DEFAULT 1,
  border_top_right_radius double precision NOT NULL DEFAULT 1,
  border_bottom_left_radius double precision NOT NULL DEFAULT 1,
  border_bottom_right_radius double precision NOT NULL DEFAULT 1,
  max_messages integer NOT NULL DEFAULT 50,
  hide_message_pattern varchar NOT NULL DEFAULT '!',
  hide_point_rewards boolean NOT NULL DEFAULT false,
  hide_links boolean NOT NULL DEFAULT true,
  link_replacements varchar NOT NULL DEFAULT '<link>',
  ban_word_replacement varchar NOT NULL DEFAULT '***',
  ban_word_filter_id uuid NULL DEFAULT NULL,
  font_family varchar NOT NULL DEFAULT '',
  nickname_font_weight integer NOT NULL DEFAULT 500,
  text_font_weight integer NOT NULL DEFAULT 400,
  font_size double precision NOT NULL DEFAULT 1,
  user_id varchar NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS chat_custom_nicknames (
  chat_settings_id uuid PRIMARY KEY,
  nickname varchar NOT NULL,
  start_color integer NOT NULL,
  end_color integer NOT NULL,
  FOREIGN KEY (chat_settings_id) REFERENCES chat_settings(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS chat_hidden_nicknames (
  chat_settings_id uuid PRIMARY KEY,
  nickname varchar NOT NULL,
  FOREIGN KEY (chat_settings_id) REFERENCES chat_settings(id) ON DELETE CASCADE ON UPDATE CASCADE
);
-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY,
  username varchar(32) NOT NULL UNIQUE,
  created_at timestamp NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS tokens (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  user_agent varchar NOT NULL,
  ip varchar(32) NOT NULL,
  authorized_at timestamp NOT NULL,
  refreshed_at timestamp NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS twitch_data (
  user_id uuid PRIMARY KEY,
  access_token varchar NOT NULL,
  refresh_token varchar NOT NULL,
  expired_at timestamp NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);
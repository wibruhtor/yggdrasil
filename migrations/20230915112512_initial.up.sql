-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.users (
  id uuid PRIMARY KEY,
  username varchar(32) NOT NULL,
  display_name varchar(32) NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS public.tokens (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL,
  user_agent varchar NOT NULL,
  ip varchar(32) NOT NULL,
  authorized_at timestamptz NOT NULL,
  refreshed_at timestamptz NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS public.twitch_data (
  user_id uuid PRIMARY KEY,
  access_token varchar NOT NULL,
  refresh_token varchar NOT NULL,
  expired_at timestamptz NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);
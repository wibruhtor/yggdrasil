-- Add up migration script here
CREATE TABLE IF NOT EXISTS ban_word_filters (
  id uuid PRIMARY KEY,
  name varchar NOT NULL,
  user_id varchar NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS ban_words (
  ban_word_filter_id uuid,
  word varchar NOT NULL,
  
  PRIMARY KEY (ban_word_filter_id, word),
  FOREIGN KEY (ban_word_filter_id) REFERENCES ban_word_filters(id) ON DELETE CASCADE ON UPDATE CASCADE
);

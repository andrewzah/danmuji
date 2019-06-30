-- raw count ->
--   count of the raw message before punctuation & whitespace
--   gets stripped.
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  message_id VARCHAR (20) UNIQUE NOT NULL,
  guild_id VARCHAR (20) NOT NULL,
  channel_id VARCHAR (20) NOT NULL,
  user_id VARCHAR (20) NOT NULL,
  hangeul_count INT NOT NULL,
  non_hangeul_count INT NOT NULL,
  raw_count INT NOT NULL,
  time TIMESTAMPTZ NOT NULL
);

CREATE INDEX by_user ON messages (user_id, guild_id, hangeul_count, non_hangeul_count);
CREATE INDEX by_user_timestamp ON messages (user_id, guild_id, hangeul_count, non_hangeul_count, time);

-- raw count ->
--   count of the raw message before punctuation & whitespace
--   gets stripped.
CREATE TABLE messages (
  id VARCHAR (20) PRIMARY KEY UNIQUE,
  guild_id VARCHAR (20) REFERENCES guilds(id) ON DELETE CASCADE,
  channel_id VARCHAR (20) REFERENCES channels(id) ON DELETE CASCADE,
  user_id VARCHAR (20) REFERENCES users(id) ON DELETE CASCADE,
  hangeul_count INT NOT NULL,
  non_hangeul_count INT NOT NULL,
  raw_count INT NOT NULL,
  time TIMESTAMPTZ NOT NULL
);

CREATE INDEX by_user ON messages (user_id, guild_id, hangeul_count, non_hangeul_count);
CREATE INDEX by_user_timestamp ON messages (user_id, guild_id, hangeul_count, non_hangeul_count, time);

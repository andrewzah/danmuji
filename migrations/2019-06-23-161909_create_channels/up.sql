CREATE TABLE channels (
  id SERIAL PRIMARY KEY,
  channel_id VARCHAR (20) NOT NULL,
  guild_id VARCHAR (20),
  name VARCHAR (100) NOT NULL,
  enabled BOOLEAN DEFAULT true
)

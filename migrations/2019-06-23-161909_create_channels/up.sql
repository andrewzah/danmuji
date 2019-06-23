CREATE TABLE channels (
  id VARCHAR (20) PRIMARY KEY UNIQUE,
  guild_id VARCHAR (20) REFERENCES guilds(id),
  name VARCHAR (100),
  enabled BOOLEAN DEFAULT true
)

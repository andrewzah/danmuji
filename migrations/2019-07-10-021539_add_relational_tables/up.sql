CREATE table guilds (
  id SERIAL PRIMARY KEY,
  guild_id VARCHAR (20) UNIQUE NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT true
);

CREATE table channels (
  id SERIAL PRIMARY KEY,
  channel_id VARCHAR (20) UNIQUE NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT true
);

CREATE table users (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR (20) UNIQUE NOT NULL,
  opt_out BOOLEAN NOT NULL DEFAULT false
);

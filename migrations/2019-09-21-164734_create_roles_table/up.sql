CREATE TABLE roles (
  id SERIAL PRIMARY KEY,
  guild_id VARCHAR (20) NOT NULL,
  role_id VARCHAR (20) NOT NULL,
  enabled BOOLEAN NOT NULL default false
);

CREATE UNIQUE INDEX guild_role ON roles (guild_id, role_id);

CREATE TABLE users (
  id VARCHAR (20) PRIMARY KEY UNIQUE,
  name VARCHAR (32),
  enabled BOOLEAN DEFAULT true,
  roles TEXT [],
  guilds TEXT [],
  channels TEXT []
)

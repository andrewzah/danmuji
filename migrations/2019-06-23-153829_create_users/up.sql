CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR (20) UNIQUE NOT NULL,
  name VARCHAR (32) NOT NULL,
  enabled BOOLEAN DEFAULT true,
  roles TEXT [],
  guilds TEXT [],
  channels TEXT []
)

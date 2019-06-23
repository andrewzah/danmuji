INSERT INTO guilds (id)
VALUES ('g01')
ON CONFLICT DO NOTHING;

INSERT INTO channels (id, guild_id)
VALUES ('c01', 'g01')
ON CONFLICT DO NOTHING;

INSERT INTO users (id, name)
VALUES ('u01', 'name')
ON CONFLICT DO NOTHING;

INSERT INTO messages
  (id, guild_id, channel_id, user_id, hangeul_count,
   non_hangeul_count, raw_count, time)
VALUES
  ('m02', 'g01', 'c01', 'u01', 1, 2, 5, now());

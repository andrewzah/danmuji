ALTER TABLE replies
DROP COLUMN guild_id;

DROP INDEX IF EXISTS guild_tag;

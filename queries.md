Queries that Admiral Yi makes/will make (once implemented):

### null guilds
Gets a list of guilds in the `messages` table that don't have a corresponding row in the `guilds` table.

```sql
SELECT DISTINCT m.guild_id
FROM messages m
LEFT OUTER JOIN guilds g
ON m.guild_id = c.guild_id
WHERE g.id IS NULL;
```

### null channels
Gets a list of channels in the `messages` table that don't have a corresponding row in the `channels` table.

```sql
SELECT DISTINCT m.channel_id
FROM messages m
LEFT OUTER JOIN channels c
ON m.channel_id = c.channel_id
WHERE c.id IS NULL;
```

### null users
Gets a list of discord users in the `messages` table that don't have a corresponding row in the `users` table.

```sql
SELECT DISTINCT m.user_id
FROM messages m
LEFT OUTER JOIN users u
ON m.user_id = u.user_id
WHERE u.id IS NULL;
```

### update roles
todo

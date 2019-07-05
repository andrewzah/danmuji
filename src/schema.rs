table! {
    messages (id) {
        id -> Int4,
        message_id -> Varchar,
        guild_id -> Varchar,
        channel_id -> Varchar,
        user_id -> Varchar,
        hangeul_count -> Int4,
        non_hangeul_count -> Int4,
        raw_count -> Int4,
        time -> Timestamptz,
    }
}

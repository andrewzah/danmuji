table! {
    channels (id) {
        id -> Int4,
        channel_id -> Varchar,
        enabled -> Bool,
    }
}

table! {
    guilds (id) {
        id -> Int4,
        guild_id -> Varchar,
        enabled -> Bool,
    }
}

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
        content -> Nullable<Text>,
    }
}

table! {
    replies (id) {
        id -> Int4,
        tag -> Text,
        url -> Text,
        guild_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_id -> Varchar,
        opt_out -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    channels,
    guilds,
    messages,
    replies,
    users,
);

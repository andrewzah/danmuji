table! {
    channels (id) {
        id -> Int4,
        channel_id -> Varchar,
        guild_id -> Nullable<Varchar>,
        name -> Varchar,
        enabled -> Nullable<Bool>,
    }
}

table! {
    guilds (id) {
        id -> Int4,
        guild_id -> Varchar,
        name -> Varchar,
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
    }
}

table! {
    roles (id) {
        id -> Int4,
        role_id -> Varchar,
        guild_id -> Varchar,
        name -> Varchar,
        color -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_id -> Varchar,
        name -> Varchar,
        enabled -> Nullable<Bool>,
        roles -> Nullable<Array<Text>>,
        guilds -> Nullable<Array<Text>>,
        channels -> Nullable<Array<Text>>,
    }
}

allow_tables_to_appear_in_same_query!(channels, guilds, messages, roles, users,);

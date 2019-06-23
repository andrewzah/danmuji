table! {
    channels (id) {
        id -> Varchar,
        guild_id -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        enabled -> Nullable<Bool>,
    }
}

table! {
    guilds (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
    }
}

table! {
    messages (id) {
        id -> Varchar,
        guild_id -> Nullable<Varchar>,
        channel_id -> Nullable<Varchar>,
        user_id -> Nullable<Varchar>,
        hangeul_count -> Int4,
        non_hangeul_count -> Int4,
        raw_count -> Int4,
        time -> Timestamptz,
    }
}

table! {
    roles (id) {
        id -> Varchar,
        guild_id -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        color -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        enabled -> Nullable<Bool>,
        roles -> Nullable<Array<Text>>,
        guilds -> Nullable<Array<Text>>,
        channels -> Nullable<Array<Text>>,
    }
}

joinable!(channels -> guilds (guild_id));
joinable!(messages -> channels (channel_id));
joinable!(messages -> guilds (guild_id));
joinable!(messages -> users (user_id));
joinable!(roles -> guilds (guild_id));

allow_tables_to_appear_in_same_query!(
    channels,
    guilds,
    messages,
    roles,
    users,
);

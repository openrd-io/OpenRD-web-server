// @generated automatically by Diesel CLI.

diesel::table! {
    chat_groups (id) {
        id -> Integer,
        #[max_length = 64]
        biz_id -> Varchar,
        #[max_length = 64]
        user_id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_flag -> Bool,
    }
}

diesel::table! {
    chat_messages (id) {
        id -> Integer,
        #[max_length = 64]
        biz_id -> Varchar,
        #[max_length = 64]
        group_id -> VarChar,
        #[max_length = 20]
        role -> Varchar,
        content -> Text,
        tokens -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_flag -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        biz_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        mobile -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        mobile_verified -> Bool,
        email_verified -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_flag -> Bool,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chat_groups,
    chat_messages,
    users,
);

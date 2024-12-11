// @generated automatically by Diesel CLI.

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
    }
}

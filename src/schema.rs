// @generated automatically by Diesel CLI.

diesel::table! {
    modules (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        full_name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        #[max_length = 2048]
        api_url -> Varchar,
        #[max_length = 2048]
        web_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

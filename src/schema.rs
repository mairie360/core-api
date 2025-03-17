// @generated automatically by Diesel CLI.

diesel::table! {
    api_instances (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        #[max_length = 8]
        protocol -> Varchar,
        #[max_length = 64]
        host -> Varchar,
        port -> Int4,
        #[max_length = 256]
        path -> Varchar,
        module_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    modules (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    web_instances (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        #[max_length = 8]
        protocol -> Varchar,
        #[max_length = 64]
        host -> Varchar,
        port -> Int4,
        #[max_length = 256]
        path -> Varchar,
        module_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(api_instances -> modules (module_id));
diesel::joinable!(web_instances -> modules (module_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_instances,
    modules,
    web_instances,
);

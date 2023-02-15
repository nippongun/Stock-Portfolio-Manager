// @generated automatically by Diesel CLI.
diesel::table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        api_key -> Text,
    }
}

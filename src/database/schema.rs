// @generated automatically by Diesel CLI.

diesel::table! {
    stocks (id) {
        id -> Integer,
        user_id -> Integer,
        ticker -> Text,
        amount -> Integer,
        purchase_price -> Float,
        current_price -> Float,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        api_key -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(stocks, users,);

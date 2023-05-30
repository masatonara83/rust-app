// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 64]
        email -> Varchar,
    }
}

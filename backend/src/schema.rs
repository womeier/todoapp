// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        done -> Bool,
        label -> Text,
    }
}

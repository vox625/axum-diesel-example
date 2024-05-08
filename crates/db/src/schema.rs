// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        description -> Text,
        completed -> Bool,
    }
}

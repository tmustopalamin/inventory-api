// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        name -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

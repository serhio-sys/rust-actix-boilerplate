diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
    }
}

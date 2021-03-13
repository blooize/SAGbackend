table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
        summary -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

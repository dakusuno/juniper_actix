table! {
    products (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        price -> Numeric,
    }
}

table! {
    todos (id) {
        id -> Int4,
        task -> Varchar,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name_all -> Varchar,
        email -> Varchar,
    }
}

joinable!(products -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    todos,
    users,
);

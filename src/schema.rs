// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Varchar,
        user_id -> Int4,
        post_id -> Int4,
    }
}

diesel::table! {
    customers (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        subtitle -> Varchar,
    }
}

diesel::table! {
    reservations (id) {
        id -> Int4,
        reservation_date -> Date,
        party_size -> Int4,
        table_id -> Int4,
        customer_id -> Int4,
    }
}

diesel::table! {
    restaurants (id) {
        id -> Int4,
        name -> Varchar,
        address -> Varchar,
        cuisine_type -> Varchar,
    }
}

diesel::table! {
    tables (id) {
        id -> Int4,
        seating_capacity -> Int4,
        restaurant_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        full_name -> Varchar,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(reservations -> tables (table_id));
diesel::joinable!(reservations -> users (customer_id));
diesel::joinable!(tables -> restaurants (restaurant_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    customers,
    posts,
    reservations,
    restaurants,
    tables,
    users,
);

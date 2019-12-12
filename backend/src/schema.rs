table! {
    passwords (password_id) {
        password_id -> Nullable<Integer>,
        password -> Varchar,
        verification_code -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        password_id -> Nullable<Integer>,
        user_type -> Tinyint,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}

table! {
    user_types (user_type_id) {
        user_type_id -> Tinyint,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

joinable!(users -> passwords (password_id));
joinable!(users -> user_types (user_type));

allow_tables_to_appear_in_same_query!(
    passwords,
    users,
    user_types,
);

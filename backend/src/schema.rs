table! {
    debtor_fees (debtor_fee_id) {
        debtor_fee_id -> Nullable<Integer>,
        debtor_id -> Integer,
        fee_id -> Integer,
        additional_info -> Nullable<Varchar>,
        paid -> Bool,
    }
}

table! {
    events (event_id) {
        event_id -> Nullable<Integer>,
        coordinator_id -> Nullable<Integer>,
        event_type_id -> Tinyint,
        name -> Varchar,
        additional_info -> Nullable<Varchar>,
        location -> Varchar,
        event_time -> Timestamp,
        points -> Float,
    }
}

table! {
    event_files (file_id) {
        file_id -> Nullable<Integer>,
        event_id -> Integer,
        additional_info -> Nullable<Varchar>,
    }
}

table! {
    event_types (event_type_id) {
        event_type_id -> Tinyint,
        name -> Varchar,
        description -> Varchar,
        default_points -> Float,
    }
}

table! {
    fees (fee_id) {
        fee_id -> Integer,
        fee_type_id -> Tinyint,
        name -> Varchar,
        description -> Varchar,
        due_date -> Timestamp,
        fee -> Float,
    }
}

table! {
    fee_types (fee_type_id) {
        fee_type_id -> Tinyint,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    files (file_id) {
        file_id -> Integer,
        uploader -> Integer,
        audience -> Tinyint,
        file_name -> Varchar,
        description -> Varchar,
    }
}

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
        points -> Float,
    }
}

table! {
    user_attendences (user_attendence_id) {
        user_attendence_id -> Integer,
        user_id -> Integer,
        event_id -> Integer,
        given_points -> Float,
        additional_info -> Nullable<Varchar>,
    }
}

table! {
    user_types (user_type_id) {
        user_type_id -> Nullable<Tinyint>,
        name -> Varchar,
        description -> Varchar,
    }
}

joinable!(debtor_fees -> fees (fee_id));
joinable!(debtor_fees -> users (debtor_id));
joinable!(event_files -> events (event_id));
joinable!(events -> event_types (event_type_id));
joinable!(events -> users (coordinator_id));
joinable!(fees -> fee_types (fee_type_id));
joinable!(files -> user_types (audience));
joinable!(files -> users (uploader));
joinable!(user_attendences -> events (event_id));
joinable!(user_attendences -> users (user_id));
joinable!(users -> passwords (password_id));
joinable!(users -> user_types (user_type));

allow_tables_to_appear_in_same_query!(
    debtor_fees,
    events,
    event_files,
    event_types,
    fees,
    fee_types,
    files,
    passwords,
    users,
    user_attendences,
    user_types,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        userId -> Int4,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        #[max_length = 255]
        provider -> Varchar,
        #[max_length = 255]
        providerAccountId -> Varchar,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        expires_at -> Nullable<Int8>,
        id_token -> Nullable<Text>,
        scope -> Nullable<Text>,
        session_state -> Nullable<Text>,
        token_type -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        userId -> Int4,
        expires -> Timestamptz,
        #[max_length = 255]
        sessionToken -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        emailVerified -> Nullable<Timestamptz>,
        image -> Nullable<Text>,
        bio -> Nullable<Text>,
        role -> Nullable<Text>,
    }
}

diesel::table! {
    verification_tokens (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamptz,
    }
}

diesel::joinable!(accounts -> users (userId));
diesel::joinable!(sessions -> users (userId));

diesel::allow_tables_to_appear_in_same_query!(accounts, sessions, users, verification_tokens,);

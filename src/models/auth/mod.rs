use crate::diesel_schema::users::{accounts, sessions, users, verification_tokens};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    #[diesel(column_name = emailVerified)]
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: i32,
    #[diesel(column_name = userId)]
    pub user_id: i32,
    pub expires: DateTime<Utc>,
    #[diesel(column_name = sessionToken)]
    pub session_token: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    #[diesel(column_name = userId)]
    pub user_id: i32,
    #[diesel(column_name = type_)]
    pub type_: String,
    pub provider: String,
    #[diesel(column_name = providerAccountId)]
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub id_token: Option<String>,
    pub scope: Option<String>,
    pub session_state: Option<String>,
    pub token_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = verification_tokens)]
#[diesel(primary_key(identifier, token))]
pub struct VerificationToken {
    pub identifier: String,
    pub expires: DateTime<Utc>,
    pub token: String,
}

// src/lib/auth_utils.rs
use crate::diesel_schema::users::accounts;
use crate::diesel_schema::users::accounts::dsl as accounts_dsl;
use crate::models::auth::Account;
use diesel::prelude::*;

#[derive(Debug)]
pub enum TokenVerifyError {
    NotFound,
    Mismatch,
    DbError(diesel::result::Error),
}

impl From<diesel::result::Error> for TokenVerifyError {
    fn from(err: diesel::result::Error) -> Self {
        TokenVerifyError::DbError(err)
    }
}

/**
 * verify_user_token:
 *   1. Looks up the user’s account row by user_id
 *   2. Compares the DB’s `id_token` to the passed-in `session_token`
 *   3. Returns Ok(()) if matched, or Err(...) otherwise
 */
pub fn verify_user_token(
    conn: &mut PgConnection,
    user_id: i32,
    session_token: &str,
) -> Result<(), TokenVerifyError> {
    // 1. Find the account row for this user
    let account = accounts_dsl::accounts
        .filter(accounts_dsl::userId.eq(user_id))
        .first::<Account>(conn)
        .map_err(TokenVerifyError::from)?;

    // 2. If the account row has an `id_token` that matches
    let db_token = match account.id_token {
        Some(ref t) => t,
        None => return Err(TokenVerifyError::NotFound),
    };

    if db_token != session_token {
        return Err(TokenVerifyError::Mismatch);
    }

    // 3. If we got here, it matches => OK
    Ok(())
}

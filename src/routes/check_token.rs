// src/routes/check_token.rs
use crate::models::auth::Account;
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize; // your Account struct

#[derive(Deserialize)]
pub struct CheckTokenPayload {
    user_id: i32,
    session_token: String,
}

#[post("/api/check_token")]
pub async fn check_token(
    pool: web::Data<DbPool>,
    payload: web::Json<CheckTokenPayload>,
) -> impl Responder {
    let mut conn = pool.get().unwrap(); // or handle error gracefully

    // Query the `accounts` table for user_id
    let result = accounts::table
        .filter(accounts::user_id.eq(payload.user_id))
        .first::<Account>(&mut conn);

    let account = match result {
        Ok(acc) => acc,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    // Compare the token in DB to the passed-in session_token
    match account.id_token {
        Some(ref db_token) if db_token == &payload.session_token => {
            // MATCH => return 200
            HttpResponse::Ok().finish()
        }
        _ => {
            // Mismatch => 401
            HttpResponse::Unauthorized().finish()
        }
    }
}

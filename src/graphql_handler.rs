// src/graphql_handler.rs

use chrono::Utc;
use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use juniper::http::GraphQLRequest;

use crate::graphql_schema::{context::Context, schema::Schema};
use crate::models::auth::User;
// 1. Import your verify_user_token + error enum
use crate::utils::auth_utils::{verify_user_token, TokenVerifyError};

/// Handles GraphQL requests by executing the query and returning the response as JSON.
pub async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    req: HttpRequest,
    data: web::Json<GraphQLRequest>,
    context_data: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract auth token from header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    // We'll need to get a DB connection for the verification
    let mut conn = context_data.db.get().map_err(|_| {
        // If we can't get a connection, return 500
        actix_web::error::ErrorInternalServerError("DB connection error")
    })?;

    // 2. If we have a token, try to decode & verify it.
    let user = if let Some(raw_token) = auth_header {
        // Example: parse the JWT to get user_id
        match parse_jwt(raw_token) {
            Ok(jwt_claims) => {
                let extracted_user_id = jwt_claims.user_id; // e.g. i32
                                                            // 3. Compare next-auth session token vs DB’s id_token
                match verify_user_token(&mut conn, extracted_user_id, raw_token) {
                    Ok(_) => {
                        // If it matches, create your user struct
                        Some(User {
                            id: extracted_user_id,
                            // fill in other fields from claims or DB
                            email: Some("test@example.com".to_string()),
                            name: Some("Test User".to_string()),
                            role: Some("user".to_string()),
                            email_verified: Some(Utc::now()),
                            image: Some("image".to_string()),
                            bio: Some("image".to_string()),
                        })
                    }
                    Err(TokenVerifyError::Mismatch) => {
                        return Ok(HttpResponse::Unauthorized()
                            .body("Session token mismatch (verify_user_token)"));
                    }
                    Err(TokenVerifyError::NotFound) => {
                        return Ok(HttpResponse::Unauthorized()
                            .body("No id_token found for user (verify_user_token)"));
                    }
                    Err(TokenVerifyError::DbError(e)) => {
                        eprintln!("DB error: {:?}", e);
                        return Ok(HttpResponse::InternalServerError().finish());
                    }
                }
            }
            Err(e) => {
                // If token is invalid or can't parse
                // Return unauthorized or treat as no user
                return Ok(HttpResponse::Unauthorized().body(format!("Invalid token: {e}")));
            }
        }
    } else {
        // No token => anonymous
        None
    };

    // 4. Now create a Juniper context that includes the user (if any)
    let ctx = Context::new(context_data.db.clone(), user);

    // 5. Execute the GraphQL request
    let res = data.execute(&schema, &ctx).await;
    Ok(HttpResponse::Ok().json(res))
}

/// Illustrative function that extracts user_id from a JWT
/// In real code, you'd use a JWT library, parse claims, etc.
fn parse_jwt(token: &str) -> Result<Claims, String> {
    // Replace this with actual JWT parsing
    // For example, using `jsonwebtoken` crate
    let claims = Claims { user_id: 1 };
    Ok(claims)
}

struct Claims {
    user_id: i32,
}

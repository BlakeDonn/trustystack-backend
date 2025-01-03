// src/graphql_handler.rs

use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use juniper::http::GraphQLRequest;

use crate::graphql_schema::{context::Context, schema::Schema};
use crate::models::auth::User;
use crate::utils::auth_utils::{verify_user_token, TokenVerifyError};

/// Handles GraphQL requests by executing the query and returning the response as JSON.
pub async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    req: HttpRequest,
    data: web::Json<GraphQLRequest>,
    context_data: web::Data<Context>,
) -> impl Responder {
    // Extract auth token from header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    // Attempt to get a DB connection
    let mut conn = match context_data.db.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().body("DB connection error");
        }
    };

    // Initialize user as None
    let mut user: Option<User> = None;

    // If a token is provided, attempt to verify it
    if let Some(raw_token) = auth_header {
        match parse_jwt(raw_token) {
            Ok(jwt_claims) => {
                let extracted_user_id = jwt_claims.user_id;
                match verify_user_token(&mut conn, extracted_user_id, raw_token) {
                    Ok(_) => {
                        user = Some(User {
                            id: extracted_user_id,
                            email: Some("test@example.com".to_string()),
                            name: Some("Test User".to_string()),
                            role: Some("user".to_string()),
                            email_verified: Some(Utc::now()),
                            image: Some("image".to_string()),
                            bio: Some("image".to_string()),
                        });
                    }
                    Err(TokenVerifyError::Mismatch) => {
                        return HttpResponse::Unauthorized().body("Session token mismatch");
                    }
                    Err(TokenVerifyError::NotFound) => {
                        return HttpResponse::Unauthorized().body("No id_token found for user");
                    }
                    Err(TokenVerifyError::DbError(_)) => {
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            }
            Err(_) => {
                return HttpResponse::Unauthorized().body("Invalid token");
            }
        }
    }

    // Create Juniper context with the user (if any)
    let ctx = Context::new(context_data.db.clone(), user);

    // Execute the GraphQL request
    let res = data.execute(&schema, &ctx).await;

    // Return the response as JSON
    HttpResponse::Ok().json(res)
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

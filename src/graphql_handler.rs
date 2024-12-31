// src/graphql_handler.rs

use chrono::Utc;

use crate::graphql_schema::{context::Context, schema::Schema};
use crate::models::auth::User;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

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

    // Simple user creation - just create a basic user if we have a token
    let user = auth_header.map(|_token| User {
        id: 1, // Hardcoded ID
        email: Some("test@example.com".to_string()),
        name: Some("Test User".to_string()),
        role: Some("user".to_string()),
        email_verified: Some(Utc::now()),
        image: Some("image".to_string()),
        bio: Some("image".to_string()),
    });

    let ctx = Context::new(context_data.db.clone(), user);
    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

// src/graphql_handler.rs

use std::sync::Arc;

use actix_web::HttpMessage;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use juniper::http::GraphQLRequest;
use log::info; // Import to access `extensions`

use crate::graphql_schema::{context::Context, schema::Schema};
use crate::models::auth::User;

/// Handles GraphQL requests by executing the query and returning the response as JSON.
pub async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    req: HttpRequest,
    data: web::Json<GraphQLRequest>,
    context_data: web::Data<Context>,
) -> impl Responder {
    // Retrieve the authenticated user from request extensions
    let user = req.extensions().get::<User>().cloned();
    println!("{:?}", user);

    match &user {
        Some(u) => info!(
            "Authenticated request from user: {}",
            u.email.as_deref().unwrap_or("no email")
        ),
        None => info!("Unauthenticated request."),
    }

    // Create GraphQL context with the user (if any)
    let ctx = Context::new(context_data.pool.clone(), user);

    // Execute the GraphQL request
    let res = data.execute(&schema, &ctx).await;

    // Return the response as JSON
    HttpResponse::Ok().json(res)
}

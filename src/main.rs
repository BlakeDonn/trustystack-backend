// src/main.rs

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use rust_backend::graphql_handler::graphql_handler;
use rust_backend::graphql_schema::context::Context;
use rust_backend::graphql_schema::schema::create_schema;
use rust_backend::middleware::logging::GraphQLLogging;
use rust_backend::middleware::timing::Timing;
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the Rust backend server...");

    // Initialize the logger with environment variables or default to "info" level
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Logger initialized");

    // Load environment variables from .env file
    dotenv().ok();
    info!("Loaded environment variables from .env");

    // Retrieve DATABASE_URL from environment variables
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => {
            println!("DATABASE_URL found: {}", url);
            info!("Database URL: {}", url);
            url
        }
        Err(_) => {
            error!("DATABASE_URL must be set in .env");
            println!("Error: DATABASE_URL must be set in .env");
            std::process::exit(1);
        }
    };

    // Set up Diesel connection pool
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = match Pool::builder().build(manager) {
        Ok(pool) => {
            info!("Database connection pool created successfully.");
            pool
        }
        Err(e) => {
            error!("Failed to create pool: {}", e);
            std::process::exit(1);
        }
    };

    // Create Juniper GraphQL schema using the new modular RootQuery
    let schema = create_schema();
    let schema = Arc::new(schema);
    info!("GraphQL schema created.");

    // Initialize GraphQL context with the database pool
    let context = web::Data::new(Context::new(pool, None));

    // Clone schema for use in server closure
    let schema_clone = schema.clone();

    // Define allowed origins
    let allowed_origins = vec!["http://localhost:3000"]; // Update with your frontend URL in production

    // Start the Actix-web server
    let server = HttpServer::new(move || {
        // Configure CORS to allow frontend access
        let _origin_clone = allowed_origins.clone();
        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                if let Some(origin_str) = origin.as_bytes().get(..) {
                    _origin_clone
                        .iter()
                        .any(|&allowed| allowed.as_bytes() == origin_str)
                } else {
                    false
                }
            })
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(Timing)
            .wrap(GraphQLLogging) // Logs GraphQL query and variables
            .wrap(cors)
            // Share the GraphQL schema with handlers
            .app_data(web::Data::new(schema_clone.clone()))
            // Share the GraphQL context with handlers
            .app_data(context.clone())
            // GraphQL endpoint
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
            // GraphQL Playground endpoint
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(playground_handler),
            )
    })
    .bind("0.0.0.0:8080");

    match server {
        Ok(server) => {
            info!("Server started successfully at http://0.0.0.0:8080");
            server.run().await
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handler to serve the GraphQL Playground UI
async fn playground_handler() -> impl Responder {
    use actix_web::HttpResponse;
    use juniper::http::playground::playground_source;

    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

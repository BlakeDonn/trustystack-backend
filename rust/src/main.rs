use actix_web::{guard, web, App, HttpServer};
use std::sync::Arc;
use rust_backend::{graphql_handler, schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rust_backend::greet();

    let schema = Arc::new(schema::create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

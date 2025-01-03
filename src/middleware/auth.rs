use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip auth for non-GraphQL endpoints or preflight requests
        if req.path() != "/graphql" || req.method() == "OPTIONS" {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        // Get the Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if let Some(token) = auth_header {
            // Validate JWT token
            let secret =
                std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
            let key = DecodingKey::from_secret(secret.as_bytes());

            match decode::<Claims>(token, &key, &Validation::default()) {
                Ok(token_data) => {
                    // Add the validated user info to request extensions
                    req.extensions_mut().insert(token_data.claims);
                    let fut = self.service.call(req);
                    Box::pin(async move { fut.await })
                }
                Err(_) => {
                    // Invalid token
                    Box::pin(
                        async move { Err(actix_web::error::ErrorUnauthorized("Invalid token")) },
                    )
                }
            }
        } else {
            // No token provided
            Box::pin(async move { Err(actix_web::error::ErrorUnauthorized("No token provided")) })
        }
    }
}

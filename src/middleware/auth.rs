// src/middleware/auth.rs

use crate::{auth::jwt::Claims, models::auth::User};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    HttpMessage, // Import to access `extensions_mut`
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use std::pin::Pin;

/// Authentication Middleware for Actix-web
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
        // Skip authentication for OPTIONS requests
        if req.method() == "OPTIONS" {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        // During development, create a guest user
        #[cfg(debug_assertions)]
        {
            let guest = User::guest();
            req.extensions_mut().insert(guest);
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        // Production authentication logic
        let cookie_name = "next-auth.session-token";
        let jwt_token = req.cookie(cookie_name).map(|cookie| {
            info!("Found JWT token in cookie: {}", cookie.value());
            cookie.value().to_owned()
        });

        match jwt_token {
            Some(token) => {
                info!("Attempting to decode JWT token.");
                // Retrieve JWT_SECRET from environment variables
                let secret = match std::env::var("JWT_SECRET") {
                    Ok(val) => val,
                    Err(_) => {
                        error!("JWT_SECRET not set in environment variables.");
                        // Create guest user instead of returning an error
                        let guest = User::guest();
                        req.extensions_mut().insert(guest);
                        let fut = self.service.call(req);
                        return Box::pin(async move { fut.await });
                    }
                };

                let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                let validation = Validation::default();

                match decode::<Claims>(&token, &decoding_key, &validation) {
                    Ok(token_data) => {
                        info!("JWT token successfully decoded.");
                        // Create a User instance from claims
                        let user = User {
                            id: token_data.claims.sub.parse().unwrap_or(0),
                            email: Some(token_data.claims.email.clone()),
                            name: Some("Test User".to_string()),
                            role: Some(token_data.claims.role.clone()),
                            email_verified: Some(chrono::Utc::now()),
                            image: Some("image_url".to_string()),
                            bio: Some("Bio".to_string()),
                        };

                        info!("Inserting user into request extensions: {:?}", user.email);
                        req.extensions_mut().insert(user);
                    }
                    Err(e) => {
                        error!("Token validation error: {:?}", e);
                        // Create guest user instead of returning an error
                        let guest = User::guest();
                        req.extensions_mut().insert(guest);
                    }
                }
            }
            None => {
                info!("No authentication token found, creating guest user.");
                let guest = User::guest();
                req.extensions_mut().insert(guest);
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}

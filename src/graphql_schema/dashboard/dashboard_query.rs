// src/graphql_schema/queries/dashboard_query.rs

use diesel::prelude::*;
use juniper::{graphql_object, FieldError, FieldResult};

use crate::diesel_schema::users::users::dsl as users_dsl;

use crate::graphql_schema::context::Context;
use crate::models::auth::User;
use crate::utils::auth_utils::{verify_user_token, TokenVerifyError};

#[derive(Clone, Debug, juniper::GraphQLObject)]
pub struct DashboardData {
    pub projects: Vec<String>,
    pub welcome_msg: String,
}

pub struct DashboardQuery;

#[graphql_object(context = "Context")]
impl DashboardQuery {
    /// Return the user's dashboard data, verifying session token first.
    async fn dashboard_data(
        context: &Context,
        user_id: i32,
        session_token: String,
    ) -> FieldResult<DashboardData> {
        // Obtain a database connection from the context
        let mut conn = context.get_connection().map_err(|e| {
            FieldError::new("Failed to get database connection", juniper::Value::null())
        })?;

        // 1. Verify token
        match verify_user_token(&mut conn, user_id, &session_token) {
            Ok(_) => { /* Token is valid; proceed */ }
            Err(TokenVerifyError::Mismatch) => {
                return Err(FieldError::new(
                    "Session token mismatch",
                    juniper::Value::null(),
                ));
            }
            Err(TokenVerifyError::NotFound) => {
                return Err(FieldError::new(
                    "No id_token found for user",
                    juniper::Value::null(),
                ));
            }
            Err(TokenVerifyError::DbError(e)) => {
                eprintln!("Database error during token verification: {:?}", e);
                return Err(FieldError::new(
                    "Internal server error during token verification",
                    juniper::Value::null(),
                ));
            }
        }

        // 2. Query your user record
        let user = users_dsl::users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|e| {
                eprintln!("Error fetching user from database: {:?}", e);
                FieldError::new("Error fetching user data", juniper::Value::null())
            })?;

        // Example "projects"
        let projects = vec!["Project A".to_string(), "Project B".to_string()];
        let user_name = user.name.unwrap_or_else(|| "Mystery User".to_string());

        // 3. Return combined data
        Ok(DashboardData {
            projects,
            welcome_msg: format!("Welcome, {}!", user_name),
        })
    }
}

// src/graphql_schema/queries/dashboard_query.rs

use diesel::prelude::*;
use juniper::{graphql_object, FieldError, FieldResult};

use crate::diesel_schema::users::users::dsl as users_dsl;

use crate::graphql_schema::context::Context;
use crate::models::auth::User;

#[derive(Clone, Debug, juniper::GraphQLObject)]
pub struct DashboardData {
    pub projects: Vec<String>,
    pub welcome_msg: String,
}

pub struct DashboardQuery;

#[graphql_object(context = "Context")]
impl DashboardQuery {
    /// Return the user's dashboard data using JWT auth
    async fn dashboard_data(context: &Context) -> FieldResult<DashboardData> {
        // Get user from context (which was set by auth middleware)
        let user = context
            .current_user
            .as_ref()
            .ok_or_else(|| FieldError::new("Not authenticated", juniper::Value::null()))?;

        // Example "projects" - you can replace this with real data
        let projects = vec!["Project A".to_string(), "Project B".to_string()];
        let user_name = user
            .name
            .clone()
            .unwrap_or_else(|| "Mystery User".to_string());

        Ok(DashboardData {
            projects,
            welcome_msg: format!("Welcome, {}!", user_name),
        })
    }
}

//// src/routes/dashboard_data.rs
//
//use actix_web::{post, web, HttpResponse, Responder};
//use diesel::prelude::*;
//use serde::{Deserialize, Serialize};
//
//use crate::diesel_schema::users::dsl as users_dsl;
//use crate::models::auth::User;
//use crate::utils::auth_utils::{verify_user_token, TokenVerifyError};
//
//#[derive(Deserialize)]
//pub struct DashboardDataPayload {
//    pub user_id: i32,
//    pub session_token: String,
//}
//
//// The shape of your "dashboard data"
//#[derive(Serialize)]
//pub struct DashboardData {
//    pub projects: Vec<String>,
//    pub welcome_msg: String,
//}
//
//#[post("/api/dashboard_data")]
//pub async fn dashboard_data(
//    pool: web::Data<DbPool>, // or however you store the Actix pool
//    payload: web::Json<DashboardDataPayload>,
//) -> impl Responder {
//    let mut conn = pool.get().expect("Failed to get DB connection");
//
//    // 1. Verify token
//    match verify_user_token(&mut conn, payload.user_id, &payload.session_token) {
//        Ok(_) => { /* token matches => proceed */ }
//        Err(TokenVerifyError::Mismatch) => {
//            return HttpResponse::Unauthorized().body("Session token mismatch");
//        }
//        Err(TokenVerifyError::NotFound) => {
//            return HttpResponse::Unauthorized().body("No id_token found for user");
//        }
//        Err(TokenVerifyError::DbError(e)) => {
//            eprintln!("DB error: {:?}", e);
//            return HttpResponse::InternalServerError().finish();
//        }
//    }
//
//    // 2. If success => gather data
//    let user_result = users_dsl::users
//        .find(payload.user_id)
//        .first::<User>(&mut conn);
//
//    let user = match user_result {
//        Ok(u) => u,
//        Err(e) => {
//            eprintln!("User fetch error: {:?}", e);
//            return HttpResponse::InternalServerError().finish();
//        }
//    };
//
//    // Example "projects"
//    let projects = vec!["Project A".to_string(), "Project B".to_string()];
//    let user_name = user.name.unwrap_or_else(|| "Mystery User".to_string());
//
//    // 3. Return combined data
//    let response = DashboardData {
//        projects,
//        welcome_msg: format!("Welcome, {}!", user_name),
//    };
//
//    HttpResponse::Ok().json(response)
//}
//

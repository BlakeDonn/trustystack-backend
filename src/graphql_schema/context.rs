// src/graphql_schema/context.rs

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use juniper::{FieldError, FieldResult};
use log::{debug, error, info};

use crate::models::auth::User;
use crate::models::parts::category::Category;
use crate::models::parts::cpu_spec::CpuSpec;
use crate::models::parts::gpu_spec::GpuSpec;
use crate::models::parts::manufacturer::Manufacturer;
use crate::models::parts::memory_spec::MemorySpec;
use crate::models::parts::storage_spec::StorageSpec;

/// Represents the context that holds the database connection pool and the authenticated user.
pub struct Context {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub current_user: Option<User>,
}

impl Context {
    /// Creates a new context with the provided database connection pool and authenticated user.
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, current_user: Option<User>) -> Self {
        let user = current_user.unwrap_or_else(User::guest);
        info!(
            "Creating new Context with user: {} ({})",
            user.name.as_deref().unwrap_or("unnamed"),
            if user.id == 0 {
                "guest"
            } else {
                "authenticated"
            }
        );
        Context {
            pool,
            current_user: Some(user),
        }
    }

    /// Retrieves a connection from the pool.
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        debug!("Attempting to get a database connection from the pool.");
        self.pool.get()
    }

    /// Fetches a manufacturer by ID from the database with authorization check.
    pub fn get_manufacturer_by_id(&self, manufacturer_id_val: i32) -> FieldResult<Manufacturer> {
        use crate::diesel_schema::parts::manufacturers::dsl::*;

        // Authorization: Only allow admins to fetch manufacturers
        if let Some(ref user) = self.current_user {
            if user.role.as_deref() != Some("admin") {
                return Err(FieldError::new(
                    "Unauthorized",
                    juniper::Value::scalar("You do not have access to this resource."),
                ));
            }
        } else {
            return Err(FieldError::new(
                "Unauthorized",
                juniper::Value::scalar("You must be logged in to access this resource."),
            ));
        }

        info!("Fetching manufacturer with ID: {}", manufacturer_id_val);
        let mut conn = self.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let manufacturer = manufacturers
            .filter(id.eq(manufacturer_id_val))
            .first::<Manufacturer>(&mut conn)
            .map_err(|e| {
                error!("Error fetching manufacturer: {}", e);
                FieldError::new(
                    "Error fetching manufacturer",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        info!("Successfully fetched manufacturer: {:?}", manufacturer);
        Ok(manufacturer)
    }

    /// Fetches a category by ID from the database.
    pub fn get_category_by_id(&self, category_id_val: i32) -> FieldResult<Category> {
        use crate::diesel_schema::parts::categories::dsl::*;

        // Example: Allow both admins and regular users to fetch categories
        if self.current_user.is_none() {
            return Err(FieldError::new(
                "Unauthorized",
                juniper::Value::scalar("You must be logged in to access this resource."),
            ));
        }

        info!("Fetching category with ID: {}", category_id_val);
        let mut conn = self.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let category = categories
            .filter(id.eq(category_id_val))
            .first::<Category>(&mut conn)
            .map_err(|e| {
                error!("Error fetching category: {}", e);
                FieldError::new(
                    "Error fetching category",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        info!("Successfully fetched category: {:?}", category);
        Ok(category)
    }

    /// Fetches a GPU specification by part ID from the database.
    pub fn get_gpu_spec_by_part_id(&self, part_id_val: i32) -> FieldResult<Option<GpuSpec>> {
        use crate::diesel_schema::parts::gpu_specs::dsl::*;

        // Authorization: Example logic, adjust as needed
        if self.current_user.is_none() {
            return Err(FieldError::new(
                "Unauthorized",
                juniper::Value::scalar("You must be logged in to access this resource."),
            ));
        }

        info!("Fetching GPU spec for part ID: {}", part_id_val);
        let mut conn = self.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let gpu_spec = gpu_specs
            .filter(part_id.eq(part_id_val))
            .first::<GpuSpec>(&mut conn)
            .optional()
            .map_err(|e| {
                error!("Error fetching GPU spec: {}", e);
                FieldError::new(
                    "Error fetching GPU spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        match &gpu_spec {
            Some(spec) => info!("Successfully fetched GPU spec: {:?}", spec),
            None => info!("No GPU spec found for part ID: {}", part_id_val),
        }

        Ok(gpu_spec)
    }

    /// Fetches a CPU specification by part ID from the database.
    pub fn get_cpu_spec_by_part_id(&self, part_id_val: i32) -> FieldResult<Option<CpuSpec>> {
        use crate::diesel_schema::parts::cpu_specs::dsl::*;

        // Authorization: Example logic, adjust as needed
        if self.current_user.is_none() {
            return Err(FieldError::new(
                "Unauthorized",
                juniper::Value::scalar("You must be logged in to access this resource."),
            ));
        }

        info!("Fetching CPU spec for part ID: {}", part_id_val);
        let mut conn = self.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let cpu_spec = cpu_specs
            .filter(part_id.eq(part_id_val))
            .first::<CpuSpec>(&mut conn)
            .optional()
            .map_err(|e| {
                error!("Error fetching CPU spec: {}", e);
                FieldError::new(
                    "Error fetching CPU spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        match &cpu_spec {
            Some(spec) => info!("Successfully fetched CPU spec: {:?}", spec),
            None => info!("No CPU spec found for part ID: {}", part_id_val),
        }

        Ok(cpu_spec)
    }

    /// Fetches a Memory specification by part ID from the database.
    pub fn get_memory_spec_by_part_id(&self, part_id_val: i32) -> FieldResult<Option<MemorySpec>> {
        use crate::diesel_schema::parts::memory_specs::dsl::*;

        // Authorization: Example logic, adjust as needed
        if self.current_user.is_none() {
            return Err(FieldError::new(
                "Unauthorized",
                juniper::Value::scalar("You must be logged in to access this resource."),
            ));
        }

        info!("Fetching Memory spec for part ID: {}", part_id_val);
        let mut conn = self.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let memory_spec = memory_specs
            .filter(part_id.eq(part_id_val))
            .first::<MemorySpec>(&mut conn)
            .optional()
            .map_err(|e| {
                error!("Error fetching Memory spec: {}", e);
                FieldError::new(
                    "Error fetching Memory spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        match &memory_spec {
            Some(spec) => info!("Successfully fetched Memory spec: {:?}", spec),
            None => info!("No Memory spec found for part ID: {}", part_id_val),
        }

        Ok(memory_spec)
    }

    /// Fetches a Storage specification by part ID from the database.
    pub fn get_storage_spec_by_part_id(
        &self,
        part_id_val: i32,
    ) -> FieldResult<Option<StorageSpec>> {
        use crate::diesel_schema::parts::storage_specs::dsl::*;

        // Authorization: Example logic, adjust as needed
        if self.current_user.is_none() {
            return Err(FieldError::new(
                "Unauthorized",
                juniper::Value::scalar("You must be logged in to access this resource."),
            ));
        }

        info!("Fetching Storage spec for part ID: {}", part_id_val);
        let mut conn = self.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let storage_spec = storage_specs
            .filter(part_id.eq(part_id_val))
            .first::<StorageSpec>(&mut conn)
            .optional()
            .map_err(|e| {
                error!("Error fetching Storage spec: {}", e);
                FieldError::new(
                    "Error fetching Storage spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        match &storage_spec {
            Some(spec) => info!("Successfully fetched Storage spec: {:?}", spec),
            None => info!("No Storage spec found for part ID: {}", part_id_val),
        }

        Ok(storage_spec)
    }
}

/// Required to implement Juniper's `Context` trait for integration with GraphQL.
impl juniper::Context for Context {}

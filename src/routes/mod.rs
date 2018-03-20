//! This model represents all the routes managed by Rocket.  
//! Some routes may not be active: you may want to check [main.rs](/src/gtfs_server/main.rs.html)
//! for a list of enabled routes.

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

pub struct RoutesHandler {
    pub pool : Pool<PostgresConnectionManager>
}

use rocket::State;
use super::rocket_contrib::Json;
use super::models::api as model_api;

pub mod api;


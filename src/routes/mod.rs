//! This model represents all the routes managed by Rocket.
//! Some routes may not be active: you may want to check [main.rs](/src/gtfs_server/main.rs.html)
//! for a list of enabled routes.

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use super::models::api as model_api;
use rocket::State;
use postgres::NoTls;

pub struct RoutesHandler {
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
}

pub mod api;
pub mod ui;

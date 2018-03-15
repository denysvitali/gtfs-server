extern crate rocket_contrib;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

pub struct RoutesHandler {
    pub pool : Pool<PostgresConnectionManager>
}

use rocket::State;

use self::rocket_contrib::Json;

use super::models::api as model_api;

pub mod api;


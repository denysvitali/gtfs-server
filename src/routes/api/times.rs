use super::model_api::tripresult::TripResult;
use super::model_api::meta::Meta;

use models::trip::Trip;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use postgres::rows::Row;

#[get("/times/<trip_id>")]
pub fn times_stop(rh: State<RoutesHandler>, trip_id: String) -> Json<TimesResult>{

}
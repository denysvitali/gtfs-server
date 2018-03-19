use rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};
use rocket_contrib::Json;
use rocket::local::LocalResponse;
use rocket::{Rocket, Route};

use models::api::resultarray::ResultArray;
use models::stop::Stop;
use models::api::stopdistance::StopDistance;

use std::prelude::*;


use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

use models::api::result::Result;

use importer::serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use importer::serde_json;
use routes::RoutesHandler;

use super::route_api as api;

fn create_pool() -> Pool<PostgresConnectionManager> {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:mysecretpassword@172.18.0.2:5432",
        TlsMode::None
    ).unwrap();
    let pool = Pool::new(manager).unwrap();
    pool
}

fn create_server(routes: Vec<Route>) -> Rocket {
    let pool = create_pool();
    let rh = RoutesHandler { pool };
    rocket::ignite()
        .manage(rh)
        .mount("/api", routes)
}

#[cfg(test)]
#[test]
pub fn main() {
    let r = create_server(routes![api::main]);

    let client = Client::new(r).expect("valid rocket instance");
    let req = client.get("/api");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[cfg(test)]
#[test]
pub fn stops_near_coordinate() {
    let r = create_server(routes![api::stops::stops_near]);

    let client = Client::new(r).expect("valid rocket instance");
    let req = client.get("/api/stops/near/46.02487/8.917352/100");
    let mut response : LocalResponse = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let ra : ResultArray<StopDistance> =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(ra.meta.success, true, "Server responded w/ success:false");
    let result = &ra.result.unwrap();
    assert_eq!(result.len() > 0, true);

    let stop : &StopDistance = result.get(0).unwrap();
    assert_eq!(stop.stop.name, "Manno, La Monda");
    assert_eq!(stop.stop.uid, "s-c27ebe-mannolamonda"); // May vary, in case of another feed SHA256sum
    assert_eq!(stop.stop.lat, 46.02487);
    assert_eq!(stop.stop.lng, 8.917352);
    assert_eq!(stop.stop.location_type, 0);
    assert_eq!(stop.distance <= 100.0, true);
}

#[cfg(test)]
#[test]
pub fn stop_by_id() {
    let r = create_server(routes![api::stops::stops_by_id]);

    let client = Client::new(r).expect("valid rocket instance");
    let req = client.get("/api/stops/s-c27ebe-mannolamonda");
    let mut response : LocalResponse = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let res : Result<Stop> =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(res.meta.success, true, "Server responded w/ success:false");

    let stop : Stop = res.result.unwrap();
    assert_eq!(stop.name, "Manno, La Monda");
    assert_eq!(stop.uid, "s-c27ebe-mannolamonda"); // May vary, in case of another feed SHA256sum
    assert_eq!(stop.lat, 46.02487);
    assert_eq!(stop.lng, 8.917352);
    assert_eq!(stop.location_type, 0);
}

#[cfg(test)]
#[test]
pub fn invalid_stop_by_id() {
    let r = create_server(routes![api::stops::stops_by_id]);

    let client = Client::new(r).expect("valid rocket instance");
    let req = client.get("/api/stops/s-c27ebe-invalid-stop-name");
    let mut response : LocalResponse = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let res : Result<Stop> =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(res.meta.success, false, "Server responded w/ success:true when an invalid stop was provided");
    assert_eq!(res.result.is_none(), true);
}
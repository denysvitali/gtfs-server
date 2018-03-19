use super::model_api::meta::Meta;

use models::route::Route;
use super::model_api::result::Result;
use super::model_api::resultarray::ResultArray;
use super::model_api::error::Error;

use super::agency;

use super::super::RoutesHandler;
use super::super::Json;
use super::super::State;
use super::super::Pool;
use super::super::PostgresConnectionManager;

use postgres::rows::Row;

#[get("/routes")]
pub fn routes(rh: State<RoutesHandler>) -> Json<ResultArray<Route>>{
    let query = "SELECT * FROM route LIMIT 50";
    let conn = rh.pool.clone().get().unwrap();
    let routes = conn.query(
        query,
        &[]
    );
    let mut routes_result: Vec<Route> = Vec::new();

    for row in routes.expect("Query failed").iter() {
        let route = parse_route_row(&row, &rh);
        routes_result.push(route);
    }

    Json(ResultArray{
        result: Some(routes_result),
        meta: Meta {
            success: true,
            error: Option::None
        }
    })
}

#[get("/routes/<route_uid>")]
pub fn route_by_id(rh: State<RoutesHandler>, route_uid: String) -> Json<Result<Route>>{
    let query = "SELECT * FROM route WHERE uid = $1 LIMIT 1";
    let conn = rh.pool.clone().get().unwrap();
    let routes = conn.query(
        query,
        &[&route_uid]
    );

    let routes = routes.expect("Query failed");

    if routes.len() != 1 {
        return Json(Result{
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error{
                    code: 4,
                    message: String::from("Unable to find this route")
                })
            }
        })
    }

    let route = parse_route_row(&routes.get(0), &rh);

    Json(Result{
        result: Some(route),
        meta: Meta {
            success: true,
            error: Option::None
        }
    })
}

fn parse_route_row(row: &Row, rh: &State<RoutesHandler>) -> Route {
    let feed_id : String = row.get(7);
    Route {
        uid: row.get(0),
        id: row.get(1),
        agency_id: agency::get_agency_id(row.get(2), &feed_id, &rh),
        short_name: row.get(3),
        long_name: row.get(4),
        description: row.get(5),
        route_type: row.get(6),
        feed_id
    }
}
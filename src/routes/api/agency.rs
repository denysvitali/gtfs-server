//! `/agency` related routes

use super::model_api::meta::Meta;

use models::agency::Agency;

use super::model_api::error::Error;
use super::model_api::result::Result;

use super::super::Json;
use super::super::RoutesHandler;
use super::super::State;

/// `/agency/<agency_uid>`  
/// Get the the specified [Agency](../../../models/agency/struct.Agency.html) by its specified UID.  
/// Returns a [Result](../../../models/api/result/struct.Result.html)<[Agency](../../../models/agency/struct.Agency.html)>
///
/// ### Example
/// `/api/agency/a-cfb94d-aroserverkehrsbetriebe` returns:
/**
    ```json
    {
      "result": {
        "uid": "a-cfb94d-aroserverkehrsbetriebe",
        "name": "Aroser Verkehrsbetriebe",
        "url": "http://www.sbb.ch/",
        "timezone": "Europe/Berlin",
        "lang": "DE",
        "phone": "0900 300 300 "
      },
      "meta": {
        "success": true
      }
    }
    ```
**/
#[get("/agency/<agency_uid>")]
pub fn agency_by_id(rh: State<RoutesHandler>, agency_uid: String) -> Json<Result<Agency>> {
    let res = get_agency_by_uid(rh, agency_uid);

    if res.is_none() {
        return Json(Result {
            result: Option::None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 3,
                    message: String::from("This agency doesn't exists"),
                }),
            },
        });
    }

    Json(Result {
        result: res,
        meta: Meta {
            success: true,
            error: Option::None,
        },
    })
}

use postgres::rows::Row;

fn get_agency_by_uid(rh: State<RoutesHandler>, agency_uid: String) -> Option<Agency> {
    let query = "SELECT \
                 uid, \
                 id, \
                 name, \
                 url, \
                 timezone, \
                 lang, \
                 phone, \
                 feed_id \
                 FROM agency \
                 WHERE uid=$1\
                 LIMIT 1";

    let conn = rh.pool.clone().get().unwrap();
    let agency = conn.query(query, &[&agency_uid]);
    let agency = agency.expect("Query failed");

    if agency.len() != 1 {
        return Option::None;
    }

    return Some(parse_agency_row(&agency.get(0)));
}

fn get_agency(
    agency_id: Option<String>,
    feed_id: &String,
    rh: &State<RoutesHandler>,
) -> Option<Agency> {
    if agency_id.is_none() {
        return Option::None;
    }
    let query = "SELECT \
                 uid, \
                 id, \
                 name, \
                 url, \
                 timezone, \
                 lang, \
                 phone, \
                 feed_id \
                 FROM agency \
                 WHERE id=$1 AND feed_id=$2 \
                 LIMIT 1";

    let conn = rh.pool.clone().get().unwrap();
    let agencies = conn.query(query, &[&agency_id, &feed_id]);

    let result = agencies.expect("Query failed");
    if result.len() != 1 {
        return Option::None;
    }

    return Some(parse_agency_row(&result.get(0)));
}

/// Returns the UID of the `agency_id` and `feed_id` provided.
pub fn get_agency_id(
    agency_id: Option<String>,
    feed_id: &String,
    rh: &State<RoutesHandler>,
) -> Option<String> {
    if agency_id.is_none() {
        return Option::None;
    }
    let query = "SELECT \
        uid
        FROM agency \
        WHERE id=$1 AND feed_id=$2 \
        LIMIT 1";

    let conn = rh.pool.clone().get().unwrap();
    let agencies = conn.query(query, &[&agency_id, &feed_id]);

    let result = agencies.expect("Query failed");
    if  result.len() != 1 {
        return Option::None;
    }

    return Some(result.get(0).get(0));
}

fn parse_agency_row(row: &Row) -> Agency {
    Agency {
        uid: row.get(0),
        id: row.get(1),
        name: row.get(2),
        url: row.get(3),
        timezone: row.get(4),
        lang: row.get(5),
        phone: row.get(6),
        feed_id: row.get(7),
    }
}

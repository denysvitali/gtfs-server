//! `/db` related routes

use super::super::Pool;
use super::super::PostgresConnectionManager;
use super::super::RoutesHandler;

use super::super::super::importer;
use super::super::Json;
use super::super::State;
use super::model_api::meta::Meta;
use super::model_api::result::Result;
use super::model_api::successresult::SuccessResult;

/// `/db/update`
/// Updates the DB schema. This operation should be performed after each update
/// because the DB may have been updated.
/// Returns a [SuccessResult](../../../models/api/result/struct.SuccessResult.html)
#[get("/db/update")]
pub fn update(rh: State<RoutesHandler>) -> Json<SuccessResult> {
    let res = importer::update_db(&rh.pool);

    Json(SuccessResult { success: true })
}

/// `/db/version`
/// Returns the current DB version
/// Returns a [Result](../../../models/api/result/struct.Result.html)<[i32](https://doc.rust-lang.org/std/primitive.i32.html)>
#[get("/db/version")]
pub fn version(rh: State<RoutesHandler>) -> Json<Result<i32>> {
    let res = importer::get_db_version(&rh.pool);

    Json(Result::<i32> {
        result: Some(res),
        meta: Meta {
            success: true,
            error: Option::None,
        },
    })
}

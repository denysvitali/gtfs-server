use super::model_api::stopresult::StopResult;
use super::model_api::meta::Meta;
use super::model_api::error::Error;

use super::super::RoutesHandler;
use super::super::State;

#[get("/stops")]
fn stops(rh: State<RoutesHandler>) -> Json<StopResult> {
    let sr = StopResult {
        result: Vec::new(),
        meta: Meta{
            success: false,
            error: Error{ code: 0, message: String::new() }
        } };
    sr
}
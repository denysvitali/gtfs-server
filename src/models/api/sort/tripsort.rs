//! Trip Sort Model

use rocket::http::RawStr;
use rocket::request::FromFormValue;
use rocket::request::{FormItems, FromForm};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TripSort {
    ArrivalTime,
    DepartureTime,
    DirectionId,
    ServiceId,
    RouteId,
    Uid,
    None,
}

impl<'f> FromFormValue<'f> for TripSort {
    type Error = ();

    fn from_form_value(form_value: &RawStr) -> Result<Self, <Self as FromFormValue>::Error> {
        Ok(match form_value.to_lowercase().as_str() {
            "arrivaltime" => TripSort::ArrivalTime,
            "departuretime" => TripSort::DepartureTime,
            "directionid" => TripSort::DirectionId,
            "serviceid" => TripSort::ServiceId,
            "routeid" => TripSort::RouteId,
            "uid" => TripSort::Uid,
            _ => TripSort::None,
        })
    }
}

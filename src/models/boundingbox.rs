//! Bouding Box struct and implementation

use models::coordinate::Coordinate;
use regex::Match;
use regex::Regex;
use rocket::http::RawStr;
use rocket::request::FromParam;

/// A Bouding Box is defined (as a parameter) as following:  
/// `p1_lat,p1_lng,p2_lat,p2_lng` where `pn_lat` is the latitude of the n-th point and
/// `pn_lng` is the longitude of the n-th point.  
///   
/// For example the string `51.2867602,-0.5103751,51.6918741,0.3340155` defines a bouding box
/// enclosing the London's area
///   
/// `45.818,5.9559,47.8084,10.4921` is the bounding box that encloses the swiss territory.
#[derive(Debug, Serialize)]
pub struct BoundingBox {
    pub p1: Coordinate,
    pub p2: Coordinate,
}

impl<'r> FromParam<'r> for BoundingBox {
    type Error = &'r RawStr;
    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let re =
            Regex::new(r"^((?:-)*\d+|(?:-)*\d+\.\d+),((?:-)*\d+|(?:-)*\d+\.\d+),((?:-)*\d+|(?:-)*\d+\.\d+),((?:-)*\d+|(?:-)*\d+\.\d+)$").unwrap();
        if re.is_match(param) {
            let caps = re.captures(param).unwrap();
            fn as_f64(c: Option<Match>) -> f64 {
                //println!("as_f64({})", c.unwrap().as_str());
                return c.unwrap().as_str().parse::<f64>().unwrap();
            }

            let p1_lat = as_f64(caps.get(1));
            let p2_lat = as_f64(caps.get(3));

            let p1_lng = as_f64(caps.get(2));
            let p2_lng = as_f64(caps.get(4));

            // Check coordinates range

            if p1_lat < -90.0 || p1_lat > 90.0 {
                return Err(param);
            }

            if p2_lat < -90.0 || p2_lat > 90.0 {
                return Err(param);
            }

            if p1_lng < -180.0 || p1_lng > 180.0 {
                return Err(param);
            }

            if p2_lng < -180.0 || p2_lng > 180.0 {
                return Err(param);
            }

            let p1 = Coordinate {
                lat: p1_lat,
                lng: p1_lng,
            };

            let p2 = Coordinate {
                lat: p2_lat,
                lng: p2_lng,
            };

            let bbox = BoundingBox { p1, p2 };
            return Ok(bbox);
        }

        Err(param)
    }
}

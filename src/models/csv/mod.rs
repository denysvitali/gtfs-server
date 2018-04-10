//! This module represents the entities as found in the CSV files.
//! These structs are only used for CSV-parsing. Therefore they are conform to the
//! [GTFS reference](https://developers.google.com/transit/gtfs/reference/).
//!
//! These structs aren't used anywhere but in the Import process. The API uses different structs
//! that are located on the [models module](/gtfs_server/models/index.html)

pub mod agency;
pub mod calendar;
pub mod feed;
pub mod route;
pub mod stop;
pub mod stoptime;
pub mod trip;

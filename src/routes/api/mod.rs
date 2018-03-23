use super::model_api;
use super::super::{NaiveTime, NaiveDate};

use rocket::response::content;
use rocket::http::ContentType;

pub mod agency;
pub mod import;
pub mod routes;
pub mod stops;
pub mod trips;
pub mod times;

use rocket;

#[get("/")]
pub fn main() -> content::Html<String> {
    return content::Html(format!(r#"<!DOCTYPE html>
    <html>
    <head>
        <title>GTFS API</title>
        <link rel="stylesheet" type="text/css" href="//fonts.googleapis.com/css?family=Open+Sans" />
        <style>
            body {{
                font-family: 'Open Sans';
            }}
        </style>
    </head>
    <body>
        <h1>GTFS Server</h1>
        <p>
            Commit: <a href="https://github.com/denysvitali/gtfs-server/commit/{0}">{0}</a> - {1}
        </p>
    </body>
    </html>
    "#,
                   env!("GIT_HASH"),
                   env!("BUILD_DATE")
    ));
}
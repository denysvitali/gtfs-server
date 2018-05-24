use super::super::{NaiveDate, NaiveTime};
use super::model_api;

use rocket::response::content;

pub mod agency;
pub mod db;
pub mod import;
pub mod routes;
pub mod stops;
pub mod stop_times;
pub mod times;
pub mod trips;

#[get("/")]
pub fn main() -> content::Html<String> {
    let mut s: String = String::from(env!("GIT_HASH"));
    s.truncate(7);
    return content::Html(format!(
        r#"<!DOCTYPE html>
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
            Commit ({2} - {1}) {0}
        </p>
            <table>
                <tr>
                    <td>
                        GitHub
                    </td>
                    <td>
                        <a href="https://github.com/denysvitali/gtfs-server/commit/{0}">{2}</a>
                    </td>
                </tr>
                <tr>
                    <td>
                        Gogs
                    </td>
                    <td>
                        <a href="https://git.ded1.denv.it/shrug/gtfs-server/commit/{0}">{2}</a>
                    </td>
                </tr>
            </table>
    </body>
    </html>
    "#,
        env!("GIT_HASH"),
        env!("BUILD_DATE"),
        s
    ));
}

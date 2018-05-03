//! `/import` UI related routes

use rocket::response::content;
use std::fs::File;
use std::io::Read;

#[get("/import")]
pub fn main() -> content::Html<String> {
    let mut f = File::open("static/dist/ui/import.html").unwrap();
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("Unable to read");
    content::Html(s)
}

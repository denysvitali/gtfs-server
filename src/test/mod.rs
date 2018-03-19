pub use super::routes::api as route_api;

pub mod api;

#[cfg(test)]
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

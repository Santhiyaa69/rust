use rocket::get;
use rocket_okapi::{openapi, openapi_get_routes};

#[openapi]
#[get("/<number>")]
fn hello_world(number: i32) -> String {
    let res = format!("Hello world number {}", number);
    println!("{}", &res);
    res
}

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    openapi_get_routes![hello_world]
}

#[macro_use] extern crate rocket;

#[cfg(test)] mod test;

use rocket::response::content;
use rocket::response::status;

#[get("/")]
fn index() -> status::Accepted<content::RawJson<&'static str>> {
    status::Accepted(content::RawJson({"{ 'health': 'ok' }"}))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

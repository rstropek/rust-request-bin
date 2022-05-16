mod request_data;
mod request_collector;

use request_collector::RequestCollector;
use rocket_dyn_templates::{Template, context};

#[macro_use]
extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[get("/king")]
fn king() -> &'static str {
    "kong"
}

#[get("/bin")]
fn bin_get() -> Template {
    Template::render("collected", context! { parent: "base".to_string() })
}

#[post("/bin")]
fn bin_post() -> Template {
    Template::render("collected", context! { parent: "base".to_string() })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        ping, 
        king, 
        bin_get, 
        bin_post,
    ])
    .attach(RequestCollector::default())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;

    #[test]
    fn collect_get() {
        let client = Client::tracked(super::rocket()).unwrap();

        // Send a GET request to the request bin
        let response = client.get("/bin").dispatch();
        assert!(response.into_string().unwrap().contains("Your request has been collected"));

        // Verify that sent request is in request list
        let response = client.get("/requests").dispatch();
        assert!(response.into_string().unwrap().contains("/bin"));
    }
}
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! This is rocket"
}

#[get("/ping")]
fn ping() -> &'static str {
    "Hello, world! This is ping"
}

#[get("/beta")]
fn beta() -> &'static str {
    "Hello, world! This is beta"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![ping])
        .mount("/", routes![beta])
}

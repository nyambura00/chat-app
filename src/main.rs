#[macro_use] extern crate rocket;

//getting request to the "/world" path
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/hello", routes![world]) //parsing handler fns eg. world()
}
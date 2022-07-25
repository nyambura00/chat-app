#[macro_use] extern crate rocket;

use rocket::tokio::sync::broadcast::{channel};

//getting request to the "/world" path
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone, FromForm, rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(channel::<Message>(1024).0) //creating a channel with 1024 slots
    .mount("/hello", routes![world]) //parsing handler fns eg. world()
}
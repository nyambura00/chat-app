#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{channel, Sender}, form::Form, State, Shutdown, response::stream::EventStream, serde::{Serialize, Deserialize}};
use rocket::response::stream::Event;

// #[get("/world")]
// fn world() -> &'static str {
//    "Hello, world!"
//}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

// endpoint to post messages
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

// events listener, can send events to the client
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![]{ //returning server-sent events
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                }
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(channel::<Message>(1024).0) //creating a channel with 1024 slots
    .mount("/", routes![post, events]) //parsing handler fns eg. world()
}
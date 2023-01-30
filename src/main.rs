pub mod help;
use help::reduce_spaces;

#[macro_use] extern crate rocket;

// Try visiting:
// http://127.0.0.1:8000/data
#[get("/data")]
fn data() -> String {
    let response = String::from("");
    return response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data])
}
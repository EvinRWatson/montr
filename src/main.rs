pub mod memory;
pub mod disk;
use memory::Memory;
use disk::Disk;

#[macro_use] extern crate rocket;

// Try visiting:
//   http://127.0.0.1:8000/data
#[get("/data")]
fn data() -> String {
    let memory_data: Memory = Memory::default();
    let disk_data: Disk = Disk::default();

    let response = "".to_string();
    return response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data])
}
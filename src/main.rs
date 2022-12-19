pub mod memory;
pub mod disk;
use memory::Memory;
use disk::Disk;
pub mod cpu;
use cpu::Cpu;

#[macro_use] extern crate rocket;

// Try visiting:
// http://127.0.0.1:8000/data
#[get("/data")]
fn data() -> String {
    let memory_data: Memory = Memory::default();
    let disk_data: Disk = Disk::default();
    let cpu_data: Cpu = Cpu::default();

    let response = Cpu::fetch();
    return response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data])
}
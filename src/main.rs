pub mod memory;
pub mod disk;
use memory::Memory;
use disk::Disk;
pub mod cpu;
use cpu::Cpu;
pub mod sys_proc;
use sys_proc::SystemProcess;
pub mod response;
use response::SysData;
pub mod help;
use help::reduce_spaces;

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

// Try visiting:
// http://127.0.0.1:8000/test
#[get("/test")]
fn test() -> String {
    let mem_data = Memory::fetch();
    println!("{}", &mem_data);
    let reduced = help::reduce_spaces(mem_data);
    println!("{}", reduced);
    let mem = Memory::extract_memory_data(reduced.clone());

    let mut response: Option<response::SysData> = None;
    

    return reduced;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data, test])
}
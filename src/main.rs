pub mod help;
use sysinfo::{ProcessExt, System, SystemExt};

#[macro_use] extern crate rocket;

// Try visiting:
// http://127.0.0.1:8000/data
#[get("/data")]
fn data() -> String {
    let sys = System::new_all();
    let mut response = String::from("");

    response += "Memory:\n";
    // RAM and swap information:
    response += &format!("total memory: {} bytes\n", sys.total_memory());
    response += &format!("used memory : {} bytes\n", sys.used_memory());
    response += &format!("total swap  : {} bytes\n", sys.total_swap());
    response += &format!("used swap   : {} bytes\n", sys.used_swap());
    
    return response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data])
}
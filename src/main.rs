use std::process::Command;

#[macro_use] extern crate rocket;

// Try visiting:
//   http://127.0.0.1:8000/data
#[get("/data")]
fn data() -> String {
    let response = get_memory_string();
    return response
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![data])
}

fn get_memory_string() -> String {
    let mem_output = Command::new("system_profiler")
    .args([
           "SPHardwareDataType",
           "|",
           "grep"
           ,"\" Memory:\""
            ])
    .output()
    .expect("failed to execute process");
    let output = String::from_utf8(mem_output.stdout.clone()).unwrap();
    println!("Out: {}", output);
    let error = String::from_utf8(mem_output.stderr.clone()).unwrap();
    println!("Error: {}", error);
    return output
}

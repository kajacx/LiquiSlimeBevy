use std::process::Command;

use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/compile", data = "<source_code>")]
fn compile(source_code: String) -> Vec<u8> {
    std::fs::write("user-plugin/src/lib.rs", source_code.as_bytes()).expect("write bytes");

    let output = Command::new("cargo")
        .arg("component")
        .arg("build")
        .arg("--release")
        .current_dir("./user-plugin")
        .output()
        .expect("run cargo component build");

    println!("Command exit code: {}", output.status);
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    // println!("{}", output);

    if output.status.success() {
        std::fs::read("user-plugin/target/wasm32-wasi/release/liquislime_user_plugin.wasm")
            .expect("should read wasm file")
    } else {
        vec![]
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, compile])
}

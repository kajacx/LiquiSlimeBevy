use std::process::Command;

use rocket::{fairing::*, http::Header, *};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, compile])
}

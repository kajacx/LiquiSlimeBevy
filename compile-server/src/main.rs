use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/compile", data = "<source_code>")]
fn compile(source_code: String) -> Vec<u8> {
    // vec![4, 5, 6]
    source_code.into_bytes()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, compile])
}

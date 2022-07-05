#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/<name>/<age>")]
fn name_and_age(name: &str, age: u32) -> String {
    if name == "Jimmy" {
        format!("Wow {} it is YOU. You are {} years old I know", name, age)
    } else {
        format!("Hello, my name is {} and I am {} years old", name, age)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/hello", routes![hello])
    .mount("/nameAndAge", routes![name_and_age])
}
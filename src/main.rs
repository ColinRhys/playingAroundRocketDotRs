#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

mod user;
use user::{User};
use rocket::serde::json::{Json, Value};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[get("/")]
fn get_users() -> Value {
    json!{[
        "User 1",
        "User 2"
    ]}
}

#[post("/", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>) -> Json<User> {
    user
}

#[delete("/<id>")]
fn delete(id: i32) -> Value {
    json!({"status": "ok"})
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![get_users, create_user, update, delete])
}
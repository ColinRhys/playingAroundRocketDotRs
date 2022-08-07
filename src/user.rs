#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String
}
use serde::Serialize;

#[derive(Serialize)]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "kassir")]
    Kassir,
}

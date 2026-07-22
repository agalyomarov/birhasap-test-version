use serde::Serialize;

#[derive(Serialize)]
pub enum UserRole {
    Admin,
    Kassir,
}

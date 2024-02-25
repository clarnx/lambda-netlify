use serde::{Deserialize, Serialize};

pub mod handlers;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserLoginData {
    pub username: Option<String>,
    pub password: Option<String>,
}

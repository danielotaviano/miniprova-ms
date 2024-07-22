use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct LoggedUser {
    pub id: i32,
}

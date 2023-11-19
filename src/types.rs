use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub username: String,
    pub domain: String,
    pub password: String
}
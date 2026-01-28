// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Status {
    Available,
    Borrowed(String), // String isliye taaki borrow karne waale ka naam store ho sake
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub status: Status,
}

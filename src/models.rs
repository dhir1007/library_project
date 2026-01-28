// src/models.rs
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Available,
    Borrowed(String), // String isliye taaki borrow karne waale ka naam store ho sake
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub status: Status,
}
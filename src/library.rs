// src/library.rs
use std::collections::HashMap;
use crate::models::{Book, Status}; // Models file se data mangwaya

// Yahan define karo taaki poori file ko pata ho
pub type LibResult = Result<(), String>;
pub struct Library {
    pub register: HashMap<u32, Book>,
}

impl Library {
    pub fn new() -> Self {
        Self { register: HashMap::new() }
    }

    pub fn add_book(&mut self, id: u32, book: Book) {
        self.register.insert(id, book);
    }

    // Yahan tumhara poora borrow aur return logic aayega...
    // 1. Borrow karne ka function
    pub fn borrow_book(&mut self, id: u32, borrower: String) -> LibResult {
        // 1. Check karo book register mein hai ya nahi
        let book = self.register.get_mut(&id)
            .ok_or(format!("ID {} ki koi book library mein nahi mili!", id))?; 
            // Ye '?' jaadu hai! Agar None hua toh yahin se Error return kar dega.

        // 2. Check karo availability
        if let Status::Borrowed(name) = &book.status {
            return Err(format!("'{}' pehle se {} ke paas hai!", book.title, name));
        }

        // 3. Success case
        book.status = Status::Borrowed(borrower);
        Ok(()) // Sab badhiya hai!

        //other type of code without using Error Handling
        // // get_mut use kiya kyunki humein status 'Available' se 'Borrowed' karna hai
        // if let Some(book) = self.register.get_mut(&id) {
        //     if book.status == Status::Available {
        //         book.status = Status::Borrowed;
        //         println!("Success: Aapne '{}' le li hai.", book.title);
        //     } else {
        //         println!("Oops: '{}' pehle se kisi aur ke paas hai.", book.title);
        //     }
        // } else {
        //     println!("Error: ID {} ki koi book nahi mili!", id);
        // }
    }

    // 2. Return karne ka function (Thinking Skill: Borrow ka ulta logic)
    pub fn return_book(&mut self, id: u32) {
        if let Some(book) = self.register.get_mut(&id) {
            book.status = Status::Available;
            println!("Thanks: '{}' wapas mil gayi.", book.title);
        } else {
            println!("Error: Ye book hamari library ki nahi hai.");
        }
    }
    // Reporting function - Humne '&self' liya kyunki humein sirf dekhna hai
    pub fn display_report(&self) {
        println!("--- Library Inventory Report ---");
        
        // .iter() humein ek 'Iterator' deta hai jo ownership nahi leta
        // Sirf references (&id, &book) deta hai
        for (id, book) in self.register.iter() {
            println!("ID: {}, Title: {}, Status: {:?}", id, book.title, book.status);
        }
        
        println!("--------------------------------");
    } // saare temporary references yahan 'DROP' ho jayenge! ✅
}
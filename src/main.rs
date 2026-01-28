mod library;
mod models;

use library::Library;
use models::{Book, Status};
use std::io::{self, Write};

fn main() {
    let mut my_lib = Library::new();
    println!("🚀 Advanced Library System 2026 Loaded!");

    loop {
        println!("\n1. Admin (Add Book) | 2. User (Borrow/Report) | 3. Exit");
        print!("👉 Choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error");
        let choice = input.trim();

        match choice {
            "1" => handle_admin(&mut my_lib),
            "2" => handle_user(&mut my_lib),
            "3" => {
                println!("Adios Amigo");
                break;
            }
            _ => println!("❌ Invalid Input!"),
        }
    }
}

fn handle_admin(lib: &mut Library) {
    println!("\n--- 🛠️ ADMIN PANEL ---");
    println!("Format: ID,Title,Author (Example: 101,Rust Pro,Hacker)");
    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();

    // Logic: String ko split karke data nikalna
    let parts: Vec<&str> = data.trim().split(',').collect();

    if parts.len() == 3 {
        // Topic 13: Parsing with error handling
        let id: u32 = match parts[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid ID!");
                return;
            }
        };

        let new_book = Book {
            title: parts[1].trim().to_string(),
            author: parts[2].trim().to_string(),
            status: Status::Available,
        };

        lib.add_book(id, new_book);
    } else {
        println!("❌ Wrong format use comma (,).");
    }
}

fn handle_user(lib: &mut Library) {
    println!("\n--- 👥 USER PANEL ---");
    lib.display_report();
    // Yahan User ka borrow logic aayega...
}

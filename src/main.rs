mod library;
mod models;

use library::Library;
use models::{Book, Status};
use std::io::{self, Write};

fn main() {
    let filename = "library.json";

    // File load karne ki koshish karo
    let mut my_lib = match Library::load_from_file(filename) {
        Ok(lib) => {
            println!("✅ Welcome back! Data loaded from {}", filename);
            lib
        }
        Err(_) => {
            println!("⚠️ No saved data found. Creating a new library.");
            Library::new()
        }
    };
    // let mut my_lib = Library::new();
    println!("🚀 Advanced Library System 2026 Loaded!");

    loop {
        println!("\n1. Admin (Add Book) | 2. User (Borrow/Report) | 3. Exit");
        print!("👉 Choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error");
        let choice = input.trim();

        match choice {
            "1" => {
                handle_admin(&mut my_lib, filename); // Admin ne book add ki, ab turant save karo
                my_lib
                    .save_to_file(filename)
                    .expect("Save failed after admin action");
            }
            "2" => {
                handle_user(&mut my_lib, filename); // User ne book borrow ki, ab turant save karo
                my_lib
                    .save_to_file(filename)
                    .expect("Save failed after user action");
            }
            "3" => {
                println!("Adios Amigo, Data saved successfully!");
                break;
            }
            _ => println!("❌ Invalid Input!"),
        }
    }
}

fn handle_admin(lib: &mut Library, filename: &str) {
    println!("\n--- 🛠️ ADMIN PANEL ---");
    println!("Enter Book (Format: ID,Title,Author):");
    print!("👉 ");
    io::stdout().flush().unwrap();

    let mut data = String::new();
    io::stdin().read_line(&mut data).unwrap();

    // String ko comma (,) se todkar ek list (Vector) mein daalna
    let parts: Vec<&str> = data.trim().split(',').collect();

    if parts.len() == 3 {
        let id: u32 = parts[0].trim().parse().expect("ID number hona chahiye!");
        let title = parts[1].trim().to_string();
        let author = parts[2].trim().to_string();

        // 1. Library mein add karo
        lib.add_book(
            id,
            Book {
                title,
                author,
                status: Status::Available,
            },
        );

        // 2. Turant file mein save karo 💾
        lib.save_to_file(filename).expect("Save fail ho gaya!");

        println!("✅ Book added and saved to JSON!");
    } else {
        println!("❌ Galat format! ID,Title,Author use karo.");
    }
}

fn handle_user(lib: &mut Library, filename: &str) {
    println!("\n--- 👥 USER PANEL ---");
    println!("1. View All Books | 2. Borrow a Book | 3. Return a Book | 4. Back");
    print!("👉 Choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read");

    match choice.trim() {
        "1" => lib.display_report(),
        "2" => {
            // Borrow logic 📖
            print!("Enter Book ID: ");
            io::stdout().flush().unwrap();
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).unwrap();
            let id: u32 = id_input.trim().parse().expect("ID number hona chahiye!");

            print!("Enter your name: ");
            io::stdout().flush().unwrap();
            let mut name_input = String::new();
            io::stdin().read_line(&mut name_input).unwrap();
            let borrower_name = name_input.trim().to_string();

            match lib.borrow_book(id, borrower_name) {
                Ok(_) => {
                    println!("✅ Success! Book borrow ho gayi.");
                    // Action ke baad SAVE 💾
                    lib.save_to_file(filename).expect("Save failed!");
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        "3" => {
            // Return logic 🔙
            print!("Enter Book ID to return: ");
            io::stdout().flush().unwrap();
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).unwrap();
            let id: u32 = id_input.trim().parse().expect("Invalid ID!");

            lib.return_book(id);
            // Action ke baad SAVE 💾
            lib.save_to_file(filename).expect("Save failed!");
        }
        "4" => return,
        _ => println!("❌ Invalid choice!"),
    }
}

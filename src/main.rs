
use std::collections::HashMap;

#[derive(Debug, PartialEq)] // PartialEq zaroori hai taaki hum '==' use kar sakein
enum Status {
    Available,
    Borrowed,
}
// 1. Data define karte hain (Struct)
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    status: Status,
}

// 2. Library struct jo hamari books ka "Register" rakhega
struct Library {
    register: HashMap<u32, Book>, // ID (u32) aur Book (Value)
}
// 1. Pehle hum ek naya type define karte hain readability ke liye
// Iska matlab: Success par kuch nahi (()), Error par ek String.
type LibResult = Result<(), String>;
impl Library {
    // Nayi khali library banane ka function
    fn new() -> Self {
        Self {
            register: HashMap::new(),
        }
    }

    // Book add karne ka function
    // &mut self isliye kyunki hum register mein changes kar rahe hain
    fn add_book(&mut self, id: u32, book: Book) {
        self.register.insert(id, book);
        println!("Book added with ID: {}", id);
    }

    
    // 1. Borrow karne ka function
    fn borrow_book(&mut self, id: u32) -> LibResult {
        // 1. Check karo book register mein hai ya nahi
        let book = self.register.get_mut(&id)
            .ok_or(format!("ID {} ki koi book library mein nahi mili!", id))?; 
            // Ye '?' jaadu hai! Agar None hua toh yahin se Error return kar dega.

        // 2. Check karo availability
        if book.status == Status::Borrowed {
            return Err(format!("'{}' pehle se borrowed hai!", book.title));
        }

        // 3. Success case
        book.status = Status::Borrowed;
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
    fn return_book(&mut self, id: u32) {
        if let Some(book) = self.register.get_mut(&id) {
            book.status = Status::Available;
            println!("Thanks: '{}' wapas mil gayi.", book.title);
        } else {
            println!("Error: Ye book hamari library ki nahi hai.");
        }
    }
    // Reporting function - Humne '&self' liya kyunki humein sirf dekhna hai
    fn display_report(&self) {
        println!("--- Library Inventory Report ---");
        
        // .iter() humein ek 'Iterator' deta hai jo ownership nahi leta
        // Sirf references (&id, &book) deta hai
        for (id, book) in self.register.iter() {
            println!("ID: {}, Title: {}, Status: {:?}", id, book.title, book.status);
        }
        
        println!("--------------------------------");
    } // saare temporary references yahan 'DROP' ho jayenge! ✅
}

fn main() {
    // 1. Pehle ek Library banani padegi (Master Object)
    let mut my_lib = Library::new();

    // 2. Ek Book ka instance banate hain
    let b1 = Book {
        title: String::from("Rust Mastery"),
        author: String::from("Hacker"),
        status: Status::Available,
    };

    // 3. Sabse zaroori: Book ko Library ke register mein daalna padega
    // yahan 'b1' ki ownership library ke paas chali gayi
    my_lib.add_book(101, b1);

    println!("Library setup complete!");

    // 4. AB borrow_book call hoga 'my_lib' par, 'my_book' par nahi!
    match my_lib.borrow_book(101) {
        Ok(_) => println!("Book successfully issued! ✅"),
        Err(e) => println!("Library Error: ❌ {}", e),
    }
    
    // 5. Check karne ke liye report dekhte hain
    my_lib.display_report();
}
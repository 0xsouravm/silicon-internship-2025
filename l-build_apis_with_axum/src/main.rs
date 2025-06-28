// Handling CSV files in Rust
// mod book;

// use crate::book::{Book, load_books_from_csv, save_books_to_csv};

// fn main() {

//     let mut book_list = load_books_from_csv().unwrap_or_else(|err| {
//         eprintln!("Failed to load books, starting with an empty list.");
//         Vec::new()
//     });

//     println!("Initial Book List: {:?}", book_list);


//     for book in book_list.iter() {
//         println!("Existing Books:");
//         println!("Book ID: {}, Title: {}, Author: {}", book.id, book.title, book.author);
//     }

//     println!("Adding a new book...");

//     let book_to_add = Book {
//         id: 2,
//         title: "Clean Code".to_string(),
//         author: "Robert C. Martin".to_string(),
//     };

//     book_list.push(book_to_add);

//     let _ = save_books_to_csv(&book_list)
//         .map_err(|e| eprintln!("Error saving books: {}", e));

//     println!("Books saved successfully.");

//     let mut book_list = load_books_from_csv().unwrap_or_else(|err| {
//         eprintln!("Failed to load books, starting with an empty list.");
//         Vec::new()
//     });
    
//     for book in book_list.iter() {
//         println!("Updated Books:");
//         println!("Book ID: {}, Title: {}, Author: {}", book.id, book.title, book.author);
//     }

//     ()
// }

// --- --- --- ---
// Basic CRUD APIs

use std::sync::{Arc, RwLock};

use axum::{extract::State, routing::{get, post}, Router};

mod book;
mod handler;

use handler::{
    add_book, list_books, get_book, update_book, delete_book, search_books
};

use crate::book::{Book, load_books_from_csv, save_books_to_csv};

/// Type alias for the application's shared state.
/// It holds an `Arc` (Atomic Reference Count) to an `RwLock` (Read-Write Lock)
/// which protects a `Vec<Book>` (vector of books). `Arc` allows multiple
/// threads to own a pointer to the data, and `RwLock` allows multiple readers
/// or a single writer at any given time, ensuring thread-safe access to the book data.
type AppState = State<Arc<RwLock<Vec<Book>>>>;

// Main function
#[tokio::main]
async fn main() {
    
    let books = load_books_from_csv().unwrap_or_else(|_| {
        eprintln!("Failed to load books, starting with an empty list.");
        Vec::new()
    });

    let shared_books = Arc::new(RwLock::new(books));

    
    // Standard Router Implementation
    let app = Router::new()
        .route("/all", get(list_books))
        .route("/new", post(add_book))
        .route("/search", get(search_books))
        .route("/{id}", get(get_book)
                        .put(update_book)
                        .delete(delete_book))
        .route("/test", get(|| async { "Test route" }))
        .with_state(shared_books);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


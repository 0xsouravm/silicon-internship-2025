use serde::{Deserialize, Serialize};


/// Represents a Book with an ID, title, and author.
/// Derives necessary traits for debugging, deserialization from CSV/JSON,
/// serialization to CSV/JSON, and cloning.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

/// Loads book data from the CSV file.
///
/// Reads the CSV file specified by `CSV_FILE`, deserializes each row into a `Book` struct,
/// and returns a vector of `Book`s.
///
/// # Returns
///
/// `Result<Vec<Book>, csv::Error>` - A `Result` containing the vector of books on success,
/// or a `csv::Error` if reading or deserializing fails.
pub fn load_books_from_csv() -> Result<Vec<Book>, csv::Error> {
    let mut reader =csv::Reader::from_path("assets/books.csv")?;
    let mut books = Vec::new();

    for result in reader.deserialize() {
        let book: Book = result?;
        books.push(book);
    }

    Ok(books)
}

/// Saves the current book data to the CSV file.
///
/// Writes the provided slice of `Book`s to the CSV file, overwriting its previous content.
///
/// # Arguments
///
/// * `books` - A slice of `Book`s to be saved.
///
/// # Returns
///
/// `Result<(), csv::Error>` - An `Ok(())` on success, or a `csv::Error` if writing or serializing fails.
pub fn save_books_to_csv(books: &[Book]) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_path("assets/books.csv")?;

    for book in books {
        writer.serialize(book)?;
    }

    writer.flush()?;
    Ok(())
}
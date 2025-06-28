fn main(){
    println!("📦 Ownership & Borrowing Examples");
    copy_eg();
    move_eg();
    demo_transfer();
    test_consume_return();
    basic_borrowing();
}

// Copy Trait Example - Stack Based Types (i32, bool, char, etc.)
fn copy_eg(){
    println!("\nCopy Trait Example:\n");

    let num = 42;
    let num_copy = num; 
    println!("Original Number: {num}, Copied Number: {num_copy}");

    let flag = true;
    let flag_copy = flag;
    println!("Original Flag: {flag}, Copied Flag: {flag_copy}");

    let ch = 'a';
    let ch_copy = ch;
    println!("Original Char: {ch}, Copied Char: {ch_copy}");
}

// Move Trait Example - Heap Based Types (String, Vec, etc.)
fn move_eg(){
    println!("\nMove Trait Example:\n");

    let text = String::from("Hello, Rust!");
    let text_moved = text.clone(); // Using clone to create a deep copy
    println!("Original Text: {text}, Moved Text: {text_moved}");

    let nums = vec![1, 2, 3, 4, 5];
    let nums_moved = nums; // Ownership is moved, nums is no longer valid
    // println!("Original Vector: {:?}", nums); // This line would cause a compile error
    println!("Moved Vector: {:?}", nums_moved);
}

// Transfer Of Ownership
fn demo_transfer(){
    println!("\nTransfer of Ownership Example:\n");

    let first = String::from("First String");
    println!("First String: {first}");

    let second = first; // Ownership of `first` is moved to `second`
    println!("Second String: {second}");

    let third = second; // Ownership of `second` is moved to `third`
    println!("Third String: {third}");
}

// Consume & Return Ownership
fn test_consume_return() {
    println!("\nConsume & Return Ownership Example:\n");

    let data = String::from("Data");
    println!("Original Data: {data}");
    // consume_data(data); // Ownership is moved to the function
    // println!("Data after consume: {data}"); // This line would cause a compile error

    let data = return_data(data); // Ownership is returned from the function
    println!("Data after return: {data}");
}

#[allow(dead_code)]
fn consume_data(data: String){
    println!("Consuming Data: {data}");
    // Data is consumed here, and ownership ends here
}

fn return_data(data: String) -> String {
    println!("Returning data: {data}");
    data // Ownership is returned to the caller
}

// Borrowing Examples (Immutable & Mutable)
fn basic_borrowing() {
    println!("\nBasic Borrowing Example:\n");

    let data = String::from("Original Data");
    println!("Data: {data}");

    // Immutable Borrowing
    let borrow1 = &data;
    let borrow2 = &data;
    println!("Immutable Borrow1: {borrow1}, Borrow2: {borrow2}");

    read_data(&data); // Passing a reference to the function
    println!("Data after read: {data}"); // Data is still valid here

    let mut mutable_data = String::from("Mutable Data");
    println!("\nMutable Data: {mutable_data}");

    // Mutable Borrowing
    let mut_borrow = &mut mutable_data;
    println!("Mutable Borrow: {mut_borrow}");

    update_data(&mut mutable_data); // Mustn't use another reference while mutable borrow is active
    // println!("Mutable Borrow after update: {mut_borrow}"); // This line would cause a compile error
    println!("Data after update: {mutable_data}"); // Data is updated here
    
    mutable_data.push_str(" - Updated Again"); // This line would cause a compile error if mut_borrow is still in scope
    println!("Data after additional update: {mutable_data}");
}

fn read_data(data: &String){
    println!("\nReading Data: {data}");
    // Data is borrowed here, and ownership remains with the caller
}

fn update_data(data: &mut String){
    println!("\nUpdating Data...");
    data.push_str(" - Updated");
    // Data is mutated here, and ownership remains with the caller
}
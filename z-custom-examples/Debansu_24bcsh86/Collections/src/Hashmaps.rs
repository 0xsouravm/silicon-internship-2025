use std::collections::HashMap;

fn main(){
    println!("🗺️ HashMap Examples");

    create_empty_hashmaps();
    insert_into_hashmap();
    get_from_hashmap();

    contains_key_in_hashmap();
    remove_from_hashmap();
    entry_into_hashmap();
    
    iter_keys();
    iter_values();
    iter_key_value();
    fancy_hashmap();
}

// Creating Empty HashMaps
fn create_empty_hashmaps() {
    println!("\nCreating Empty HashMaps");

    let ice_cream_map: HashMap<String, String> = HashMap::new(); // Using type annotation
    println!("Empty Hashmap: {:?}", ice_cream_map);
}

// Insert Elements Into HashMap
fn insert_into_hashmap(){
    println!("\nInsert Elements Into HashMap");

    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    println!("Initial HashMap: {:?}", ice_cream_map);

    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    println!("After Inserting Elements: {:?}", ice_cream_map);
}

// Getting Value By Key
fn get_from_hashmap(){
    println!("\nGetting Value By Key");
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    
    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    let res = ice_cream_map.get("Debansu"); // "".to_string() is not needed as keys are already String & get() requires a reference to the key
    println!("Get Debansu's Favorite Ice Cream Flavour: {:?}", res);
}

// Checking Key Existence
fn contains_key_in_hashmap(){
    println!("\nChecking Key Existence");

    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    let key_one = "Debansu";
    let key_two = "Debanshu";

    println!("Map Contains Key '{key_one}': {:?}", ice_cream_map.contains_key(key_one));
    println!("Map Contains key '{key_two}': {:?}", ice_cream_map.contains_key(key_two));
}

// Removing Elements From HashMap
fn remove_from_hashmap() {
    println!("\nRemoving Elements From HashMap");
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    
    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));
    ice_cream_map.insert("Debanshu".to_string(), String::from("Butter Scotch"));

    let key_to_remove = "Debanshu";
    println!("HashMap Before Removing '{key_to_remove}': {:?}", ice_cream_map);

    let removed_flavour = ice_cream_map.remove(key_to_remove); // Removes the key-value pair if it exists
    println!("Removed Flavour For '{key_to_remove}': {:?}", removed_flavour);
    println!("HashMap After Removing '{key_to_remove}': {:?}", ice_cream_map);
}

// Using Entry API
fn entry_into_hashmap() {
    println!("\nUsing Entry API");

    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    let entry = ice_cream_map.entry("Rust".to_owned()).or_insert("Rusty Vanilla".to_string()); // Else Entry(VacantEntry("Rust"))
    println!("Entry For 'Rust': {:?}", entry); 

    println!("HashMap After Entry: {:?}", ice_cream_map);
}

// Iterating Over Keys
fn iter_keys() {
    println!("\nIterating Over Keys");

    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    let keys = ice_cream_map.keys(); // Returns an iterator over the keys

    for key in keys { // In place of keys, we can also use ice_cream_map.keys()
        println!("Key: {key}");
    }
}

// Iterating Over Values
fn iter_values() {
    println!("\nIterating Over Values");

    let mut ice_cream_map: HashMap<String, String> = HashMap::new();
    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    let values = ice_cream_map.values(); // Returns an iterator over the values

    for value in values{ // In place of values, we can also use ice_cream_map.values()
        println!("Value: {value}");
    }
}

// Iterating Over Key-Value Pairs
fn iter_key_value() {
    println!("\nIterating Over Key-Value Pairs");
    let mut ice_cream_map: HashMap<String, String> = HashMap::new();

    ice_cream_map.insert("Debansu".to_string(), String::from("Butterscotch"));
    ice_cream_map.insert("Debadutta".to_owned(), String::from("Vanilla"));
    ice_cream_map.insert("Das".to_string(), String::from("Belgium Chocolate"));

    for (key, value) in &ice_cream_map{
        println!("Key: {key}, Value: {value}");
    }
}

// Fancy HashMap: Complex Key-Value Types
fn fancy_hashmap() {
    println!("\nFancy HashMap: Complex Key-Value Types");
    let mut fancy_map: HashMap<Vec<i32>, Vec<bool>> = HashMap::new();

    let key = vec![1, 2, 3];
    let value = vec![true, false];

    fancy_map.insert(key, value); // Inserting a vector as a key and another vector as a value
    println!("Fancy HashMap: {:?}", fancy_map); // Printing the HashMap with complex key-value types
}
fn main(){
    println!("📦 Tuple Examples");

    init_tuples();
    access_elements();
    destructure_tuples();

    let res = return_tuple();
    println!("\nReturned Tuple: {:?}", res);
    println!("First Element: {}, Second Element: {}", res.0, res.1);

    match_tuple();
    tuple_traits();
}

// Initialization
fn init_tuples(){
    println!("\nInitialization of Tuples");

    let tup1 = (1, 2, 3, 4, 5);
    println!("Tuple (Homogeneous): {:?}", tup1);

    let tup2: (i32, f64, bool, char) = (1, 2.6, true, 'f');
    println!("Tuple (Heterogeneous): {:?}", tup2);

    let unit: () = ();
    println!("Empty Tuple: {:?}", unit);

    let nested = (
        (1, 't'), 
        ([1, 1, 1], false, 3.92),
        [1, 2, 3, 4, 5]
    );
    println!("Nested Tuple: {:?}", nested);
}

// Accessing Elements
fn access_elements(){
    println!("\nAccessing Tuple Elements");

    let tup = (1, 2.6, true, 'f');
    println!("First Element: {}", tup.0);
    println!("Second Element: {}", tup.1);
    println!("Third Element: {}", tup.2);
    println!("Fourth Element: {}", tup.3);

    // println!("Out Of Bounds Element: {}", tup.4); // This will cause a compile-time error
}

// Destructuring Tuples
fn destructure_tuples() {
    println!("\nDestructuring Tuples");

    let tup = (1, 2.6, true, 'f');
    let (a, b, c, d) = tup;
    println!("Destructured Elements: a = {a}, b = {b}, c = {c}, d = {d}");

    let tup = (12, 36, 49, 72, 82, 95, 100, 120, 150, 200);
    let (.., last) = tup; // Using .. to ignore the first elements
    println!("Tuple: {:?}, Last Element: {last}", tup);
}

// Returning a Tuple from a Function
fn return_tuple() -> (i32, bool) {
    println!("\nReturning a Tuple from a Function");
    (30, false)
}

// Pattern Matching with Tuples
fn match_tuple(){
    println!("\nTuple Pattern Matching");

    let point = (-2, 10);
    match point{
        (0, 0) => println!("Origin"),
        (0, y) => println!("on Y-axis at {y}"),
        (x, 0) => println!("on X-axis at {x}"),
        (x, y ) => println!("Point At ({x}, {y})")
    }
}

// Tuple Traits Explanation
fn tuple_traits() {
    println!("\nTuple Traits");

    // Tuples implement traits like Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord
    // This allows for easy printing, cloning, and comparison of tuples.
    let tup1 = (1, 2.6);
    let tup2 = (1, 2.6);

    println!("Tuple 1: {:?}, Tuple 2: {:?}", tup1, tup2);
    println!("Are tuples equal? {}", tup1 == tup2);

    let tup3 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("\nTuple: {:?}", tup3);

    // let tup4 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Tuple: {:?}", tup4); // This will cause a compile-time error due to tuple size limit
    println!("Tuples can have a maximum of 12 elements in Rust.") // Tuples can have a maximum of 12 elements in Rust.
}
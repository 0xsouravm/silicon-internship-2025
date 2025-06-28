fn main(){
    println!("🧮 Array Examples");

    init_explicit();
    init_repeated();
    init_mutation();
    access_elements();
    get_len();

    iterate_simple();
    iterate_string();
    iterate_index();
    iterate_mutably();

    array_slicing();
    find_max();
    find_min();
}

// Initialization (Explicit)
fn init_explicit(){
    let arr = [1, 2, 3, 4, 5];
    println!("\nExplicit Initialization: {:?}", arr);
}

// Initialization (Repeated)
fn init_repeated(){
    let arr = [24; 20];
    println!("Repeated Initialization: {:?}", arr);
}

// Mutation
fn init_mutation(){
    let mut arr = [1, 2, 3, 4, 5];
    println!("\nBefore Mutation: {:?}", arr);

    arr[2] = 10; // Mutating the third element
    println!("After Mutation: {:?}", arr);   
}

// Accessing Elements
fn access_elements(){
    let arr = [1, 2, 3, 4, 5];

    println!("\nAccessing Elements:");
    println!("First Element: {}", arr[0]);
    println!("Second Element: {}", arr[1]);
    println!("Third Element: {}", arr[2]);
    println!("Fourth Element: {}", arr[3]);   
    println!("Fifth Element: {}", arr[4]);

    // println!("Last Element: {}", arr[arr.len() - 1]);
    // println!("Error Example: {}", arr[5]); // This will cause a panic at runtime
}

// Array Length
fn get_len(){
    let arr = ['1', '2', '3', 'ñ', '😎'];
    let len = arr.len();
    println!("\nArray: {:?}, Length: {len}", arr);
} 

// Simple Iteration (With Various Methods)
fn iterate_simple(){
    let arr = [1, 2, 3, 4, 5];
    println!("\nInteger Array Elements:");
    
    println!("Iterating by Value:");
    for ele in arr{
        println!("Element: {ele}");
    }

    println!("\nIterating with Reference:");
    for ele in &arr{
        println!("Element: {ele}");
    }

    println!("\nIterating with Reference (Explicit):");
    for ele in &arr{
        println!("Element: {}", &ele);
    }
}

// String Array Initialization & Iteration
fn iterate_string(){
    let string_arr = [ 
        String::from("Hello"),
        String::from("World"),
        String::from("Rust") 
    ];
    
    println!("\nString Array Elements:");
    for ele in string_arr{
        println!("Element: {ele}");
    }

    let str_arr = ["Hello", "World", "Rust"];
    println!("\nString Slice Array Elements:");

    for ele in str_arr{
        println!("Element: {ele}");
    }
}

// Iteration with Index
fn iterate_index(){
    let arr = [1, 2, 3, 4, 5];
    println!("\nIterating with Index:");
    
    for i in 0..arr.len(){
        println!("Element at index {i}: {}", arr[i]);
    }
}

// Mutable Iteration
fn iterate_mutably(){
    println!("\nMutable Iteration:");

    let mut arr = [1, 2, 3, 4, 5];
    println!("Before Mutation: {:?}", arr);

    for ele in &mut arr{
        *ele += 1; // Dereferencing `ele` to mutate the value it points to.
    }
    println!("After Mutation: {:?}", arr);
    
    for i in 0..arr.len(){
        arr[i] *= arr[i]; // Squaring each element
    }
    println!("After Squaring: {:?}", arr);
}

// Array Slicing 
fn array_slicing(){
    println!("\nArray Slicing:"); 
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    let slc: &[i32] = &arr[1..4]; // Slicing from index 1 to 3 (exclusive)
    println!("Sliced Array: {:?}", slc);
}

// Finding Maximum
fn find_max(){
    println!("\nFinding Maximum:");
    let arr = [1, 2, 3, 4, 5];
    let mut max = arr[0];

    for num in arr{
        if num > max{
            max = num;
        }
    }
    println!("Maximum Element: {max}");
}

// Finding Minimum
fn find_min(){
    println!("\nFinding Minimum:");
    let arr = [1, 2, 3, 4, 5];
    let mut min = arr[0];

    for i in 1..arr.len(){
        min = if arr[i] < min { arr[i] } else { min };
    }
    println!("Minimum Element: {min}");
}
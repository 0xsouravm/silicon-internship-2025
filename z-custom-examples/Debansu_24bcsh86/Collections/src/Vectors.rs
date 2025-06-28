fn main(){
    println!("🧺 Vector Examples");

    let vec = return_vec();
    println!("\nReturned Vector: {:?}", vec);

    create_vectors();
    push_ele();
    insert_ele();

    pop_ele();
    delete_ele();

    len_vec();
    iter_vec();
}

// Return A Vector
fn return_vec() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

// Creatng Vectors (Empty, With Values, Repeated Values)
fn create_vectors() {
    println!("\nCreating Vectors");

    // Empty Vector
    let emp_vec1: Vec<i32> = Vec::new(); // Using type annotation
    println!("Empty Vector: {:?}", emp_vec1);
    
    let emp_vec2 = Vec::<i32>::new(); // Turbo Fish Operator
    println!("Empty Vector (Turbo Fish): {:?}", emp_vec2);

    let emp_vec3: Vec<f64> = vec![]; // Using vec! macro
    println!("Empty Vector (vec! macro): {:?}", emp_vec3);

    // Vector with Initial Values
    let vec_with_val = vec![1, 2, 3, 4, 5];
    println!("\nVector With Initial Values: {:?}", vec_with_val);

    // Vector with Repeated Values
    let vec_reap = vec![10; 10]; // Vector with 10 elements, each initialized to 10
    println!("Vector With Repeated values: {:?}", vec_reap);
}

// Pushing Elements To A Vector
fn push_ele(){
    println!("\nPushing Elements to a Vector");
    
    let mut vector: Vec<i32> = vec![];
    println!("Initial Vector: {:?}", vector);

    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);
    vector.push(100);

    println!("After Pushing Elements: {:?}", vector);
}

// Insert Elements At Specific Index
fn insert_ele(){
    println!("\nInserting Elements at Specific Index");

    let mut vector = vec![1, 2, 3, 4, 5, 6];
    println!("Initial Vector: {:?}", vector);

    let ele = 100;
    let ind = 4; 

    vector.insert(ind, ele);
    println!("After Inserting {ele} at index {ind}: {:?}", vector);

    vector.insert(0, 50); // Inserting at the beginning
    vector.insert(vector.len(), 200); // Inserting at the end

    println!("Final Vector After Insertions: {:?}", vector);
}

// Poping Elements From A Vector
fn pop_ele() {
    println!("\nPopping Elements from a Vector");

    // Popping the last element from a vector
    let mut vector = vec![10, 20, 30, 40, 50];
    println!("Initial Vector: {:?}", vector);

    let popped_ele = vector.pop(); // Pops the last element
    println!("Final Vector: {:?}, Popped Element: {:?}", vector, popped_ele);

    // Popping from an empty vector
    let mut vector: Vec<i32> = vec![];
    println!("Empty Vector: {:?}", vector);

    let popped_ele = vector.pop(); // This will return None
    println!("Empty Vector After Popping: {:?}, Popped Element: {:?}", vector, popped_ele);
}

// Deleting Elements At Specific Index
fn delete_ele(){
    println!("\nDeleting Elements at Specific Index");

    let mut vec = vec![0, 1, 1, 2, 3, 5, 8, 13, 21];
    println!("Initial Vector: {:?}", vec);

    let ind = 2; // Index to delete
    let removed_ele = vec.remove(ind); // Removes the element at index 2

    println!("final Vector After Deletion: {:?}, Removed Element: {removed_ele}", vec); // Vector elements Will be shifted left
}

// Vector Length
fn len_vec(){
    println!("\nLength of a Vector");
    let vec = return_vec();

    let len = vec.len();
    println!("Vector: {:?}, Length: {len}", vec);
}

// Vector Iteration
fn iter_vec() {
    println!("\nIterating Over a Vector");

    let vec = vec![
        String::from("Hello"), 
        String::from("World"), 
        String::from("Debansu"),
        String::from("This"),
        String::from("Side!!!")
    ];
    println!("Vector: {:?}", vec);

    // println!("\nIterating by Value:");
    // for ele in vec{ // Vec will be moved here, so it can't be used after this
    //     println!("Element: {ele}");
    // }

    println!("\nIterating with Reference:");
    for ele in &vec{
        println!("Element: {ele}");
    }

    println!("\nIterating with Reference (Explicit):");
    for ele in &vec{
        println!("Element: {}", &ele);
    }
}
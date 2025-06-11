// --------------------------------------- In Class codebase ------------------------------------------- //

const PI: f32 = 3.14159; // Constant value of PI

fn main() {
    datatypes();
    homework();

    // Variables in Rust

    // // let variable_name: type = "value";
    
    // // i8
    // let a: i8 = 100;
    // // println!("a: {}", a);

    // let x: i8 = 127;
    // // println!("x: {}", x);

    // // u128
    // let b: u128 = 9999999999999999999999999999999999999;

    // // Type Inference
    // let c = -1;
    // let c = -1.968;

    // Mutability
    // let x = 100;
    // x = 40; Not Allowed
    // println!("x: {}", x);

    // let mut y = 420;
    // println!("y before: {}", y);
    // y = 100;
    // println!("y after: {}", y);

    // Shadowing
    // let z = 10; // Immutable
    // println!("z before: {}", z);
    // let z = 100; // Shadowed
    // println!("z after shadowing: {}", z);
    // let z = "Hello"; // Shadowed again
    // println!("z after shadowing again: {}", z);


    // Mutability vs Shadowing
    let mut a = 5; // Integer - i32
    a = 10; // Mutated
    // a = 1.43; // This will cause a type mismatch error

    let mut a = 4.2; // Works, because it's a new variable(Shadowed)

    // let x;
    // x = 100; // First Assignment
    // x = 200; // This will cause an error because `x` is immutable after its first assignment
    // println!("x: {}", x);

    // let mut x;
    // x = 1000; // First Assignment
    // x = 40; // Second Assignment (Mutation)
    // println!("x after mutation: {}", x);

    // let mut x;
    // x = 10;
    // println!("x: {}", x);
    // x = 3.14;
    // println!("x after: {}", x);

    println!("PI: {}", PI);
}

fn homework() {
    // u8, u16, u32, u64, u128

    // u8 -> u16 -> u32 -> u64 -> u128
    // u128 -> u64 -> u32 -> u16 -> u8

    // Static Variables
    // Type Casting

    // cargo run --bin variables
}
// --------------------------------------- In Class codebase ------------------------------------------- //

fn datatypes() {
    //  divideed into 2 types : 1. scalar 2. compound (group of multiple values)
    // scalar represents single value 
    // most used 4 types : Integers, floats, booleans, characters

    // ** in flaots f32 and f64 got the same speed in most cases, but f64 is more precise (class 4 maths :/)
    
    // compound types : tuples, arrays

    // aarrays are often compared with vectors due to the capabilities of vectors to grow and shrink in size, while arrays have a fixed size. 
    // arrays are allocated on stack and the vvery reason of using it is becus of allocation speed which is faster than heap allocated vectors.

    // --------------------------------------- In Class codebase ------------------------------------------- //

    // // Floats
    // let floating_num = 3.14;
    // // println!("floating_num: {}", floating_num);

    // let f1: f32 = 0.1;
    // let f2: f32 = 0.2;
    // let f3: f64 = (f1 + f2).into();
    // // println!("f3: {}", f3);

    // // Booleans
    // let is_true = true;
    // let is_false = false;

    // // println!("is_true: {}, is_false: {}", !is_true, !is_false);

    // // Characters
    // let char_a = 'A';
    // // println!("char_a: {}", char_a);

    // let char_n = 'ñ';
    // // println!("char_a: {}", char_n);

    // let emoji = '😊';
    // // println!("emoji: {}", emoji);

    // let char_a_bytes = "A".as_bytes();
    // let char_n_bytes = "ñ".as_bytes();
    // let emoji_bytes = "😊".as_bytes();

    // println!("Char a bytes: {:?}", char_a_bytes);
    // println!("Char n bytes: {:?}", char_n_bytes);
    // println!("Emoji bytes: {:?}", emoji_bytes);

    // --------------------------------------- In Class codebase ------------------------------------------- //

    //  Examples : Tuples
    // let tuple1 = (1, 2, 3);
    // let tuple2 = (1, 2.0, "hello sir");
    // let tuple3 = (1, 2.0, "hello sir", true);
    // let tuple4 = (1, 2.0, "hello sir", true, 'A');

    // println!("tuple1: {:?}", tuple1);
    // println!("tuple2: {:?}", tuple2);
    // println!("tuple3: {:?}", tuple3);
    // println!("tuple4: {:?}", tuple4);

    // Examples : Arrays
    // let array1 = [1, 2, 3];
    // let array2 = [1, 2, 3, 4, 5];
    // let array3 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];


}
// Variables & Data Types

fn variables(){
    // Basic Types & Variables
    println!("🔤 Basic Types & Variables:");
    
    // Signed Integer
    let a: i8 = -10;
    let b: i16 = -1000;
    let c: i32 = -100000;
    let d: i64 = -10000000000;
    let e: i128 = -10000000000000000000;
    println!("\nSigned Integers:\na = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    // Unsigned Integer
    let f: u8 = 10;
    let g: u16 = 1000;
    let h: u32 = 100000;
    let i: u64 = 10000000000;
    let j: u128 = 10000000000000000000;
    println!("\nUnsigned Integers:\nf = {}, g = {}, h = {}, i = {}, j = {}", f, g, h, i, j);

    // Floating Point
    let k: f32 = 1.24;
    let l: f64 = 1.23456789;
    println!("\nFloating Points:\nk = {}, l = {}", k, l);

    // Boolean
    let is_true = true;
    let is_false = false;
    println!("\nBooleans:\nis_true = {}, is_false = {}", is_true, is_false);

    // Character
    let char_a: char = 'a';
    let char_ñ: char = 'ñ';
    let char_emoji: char = '😊';
    println!("\nCharacters:\nchar_a = {}, char_ñ = {}, char_emoji = {}", char_a, char_ñ, char_emoji);

    // Character Bytes
    let char_a_bytes = "a".as_bytes();
    let char_ñ_bytes = "ñ".as_bytes();
    let char_emoji_bytes = "😊".as_bytes();
    println!("\nCharacter Bytes:\nchar_a_bytes = {:?}\n, char_ñ_bytes = {:?}\n, char_emoji_bytes = {:?}", char_a_bytes, char_ñ_bytes, char_emoji_bytes);
}

fn type_n_conv () {
    // Type Inference
    let a = -1;
    println!("\nInteger Value Of A: {a}");
    let a = -1.968;
    println!("Float Value Of A: {a}");

    // Type Conversion
    let f1: f32 = 0.1; 
    let f2: f32 = 0.2;
    let f3: f64 = (f1 + f2).into();
    println!("\nf3 = {f3}");
}

fn mut_n_const() {
    // Shadowing
    let x = 10;
    println!("\nInitial Value Of X: {x}");
    let x = 20;
    println!("Final Value Of X: {x}");

    // Mutability
    let mut y = 100;
    println!("\nInitial Value Of Y: {y}");
    y = 200;
    println!("Final Value Of Y: {y}");
}

const PI: f32 = 3.14159;
const GRAVITY: f64 = 9.80665;

fn constants() {
    println!("\nConstants");
    println!("PI: {PI}, g: {GRAVITY}");
}

fn overflow_examples() {
    println!("\nOverflow Examples");

    // Unsigned OverFlow
    let u: u8 = 255;
    println!("\nu8 max: {u}, wrapping_add(1): {}", u.wrapping_add(1));

    // Signed Overflow
    let i: i8 = 127;
    println!("\ni8 max: {i}, wrapping_add(1): {}", i.wrapping_add(1));
}

fn main() {
    variables();
    type_n_conv();
    mut_n_const();
    constants();
    overflow_examples();
}
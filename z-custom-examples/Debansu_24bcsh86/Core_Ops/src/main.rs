mod calculator;
mod helper;   
mod tests;

use crate::calculator::multiply;

fn main(){
    println!("🎯 Welcome To The Rust Calculator!");

    let result = calculator::simple_add(5, 10);
    println!("\nAddition Result: {result}");

    let message = calculator::add_with_greeting(5, 10, "Debansu");
    println!("{message}");

    let res = calculator::is_even(10);
    println!("Is 10 even?: {res}");

    let mul = calculator::multiply(5, 6);
    println!("Multiplication Result: {mul}");

    let mul_two = multiply(10, 20);
    println!("Multiplication Result: {mul_two}");
}
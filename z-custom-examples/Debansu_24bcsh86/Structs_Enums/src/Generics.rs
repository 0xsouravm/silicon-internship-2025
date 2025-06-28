use std::fmt::Debug;

fn main(){
    println!("📦 Generics In Rust");

    compare_i8(5, 10);
    compare_f64(5.0, 3.7);

    compare(42, 17);
    compare("Apple", "Banana");

    compare_hashmap();
    print_anything("If it compiles, then it works!!!");

    test_point();
    test_shape();
    generic_func(10, 20);
}

// Comparison Without Generics
fn compare_i8(a: i8, b: i8){
    let max = if a > b { a } else { b };
    println!("\nMax (i8): {max}");
}

fn compare_f64(a: f64, b: f64){
    let max = if a > b { a } else { b };
    println!("\nMax (f64): {max}");
}

// Generic Comparison
fn compare<T: PartialOrd + Debug> (a: T, b: T) {
    let max = if a > b { a } else { b };
    println!("\nMax (Generic): {:?}", max);
}

// HashMap Comparison Limitations
use std::collections::HashMap;
use std::marker::PhantomData;
fn compare_hashmap(){
    let mut hm_one = HashMap::<String, i32>::new(); 
    let mut hm_two = HashMap::<String, i32>::new(); 

    hm_one.insert("Hello".to_string(), 1);
    hm_two.insert("World".to_string(), 2);

    println!("\nHashMaps don't implement 'PartialOrd' directly.");
    println!("Can't compare directly: compile-time error if tried!.");

    println!("HashMap One: {:?}", hm_one);
    println!("HashMap Two: {:?}", hm_two);

    // println!("{}", hm_one > hm_two); // It will cause an error
}

// Generic Printer Function
fn print_anything<T: Debug> (item: T) {
    println!("\nPrinting: {:?}", item);
}

// Generic Struct With Phantom
#[derive(Debug)]
struct Point<T, U, V> {
    x: T,
    y: U,
    _phantom: PhantomData<V>, // PhantomData is used to associate type V without storing it
}

fn test_point(){
    println!("\nGeneric Struct Point Example");

    let point = Point::<i32, f64, String> {
        x: 42,
        y: 3.14,
        _phantom: PhantomData,
    }; //Point { x: 10, y: 20, _phantom: PhantomData };

    println!("Point => x: {}, y: {}", point.x, point.y); // println!("Point: {:?}", point);
}

// Generic Enum
#[derive(Debug)]
enum Shape<R: Debug, S: Debug, W: Debug, H: Debug> {
    Circle(R), // radius
    Square(S), // side length
    Rectangle(W, H), // width, height
}

fn test_shape(){
    println!("\nGeneric Enum Shape Example");

    let circle: Shape<f64, i8, i8, f64> = Shape::Circle(10.0);
    let square = Shape::<f64, i8, i8, f64>::Square(5);
    let rectangle = Shape::<f64, i8, i8, f64>::Rectangle(10, 20.0);

    println!("Circle: {:?}", circle);
    println!("Square: {:?}", square);
    println!("Rectangle: {:?}", rectangle);
}

// Generic Function Example
fn generic_func<T>(a: T, b: T) 
where T: PartialOrd + PartialEq + Debug + Copy + std::fmt::Display { // Demonstrates trait bounds for equality, comparison, debug, and display
    println!("\nGeneric Function Example");
    if a == b {
        println!("{a} and {b} are equal.");
    } else {
        println!("{a} and {b} are not equal.");
    }
}
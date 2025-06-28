fn main(){
    println!("🎯 Traits In Rust");

    trait_example();
    dynamic_dispatch();
}

// Traits: Define Shared Behavior
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;

    // Default Method
    fn what_is_this(){
        println!("This is a Shape.")
    }
}

trait HasSides {
    fn has_sides() -> bool;
}

trait CopyOfHasSides {
    fn has_sides() -> bool;
}

// Structs
#[derive(PartialEq)]
struct Circle {
    radius: f64,
}

#[derive(PartialEq)]
struct Square {
    side: f64,
}

#[derive(PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Trait Implementations
impl Drawable for Circle {
    fn draw(&self) {
        println!("\nA Circle with radius {} is being drawn.", self.radius);
    }

    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("\nA Square with side {} is being drawn.", self.side);
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }

    // Overriding Default Method/Function
    fn what_is_this() {
        println!("This is a Square.");
    }
}       

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("\nA Rectangle with width {} and height {} is being drawn.", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}   

// Trait Implementations (Multiple)
impl HasSides for Circle {
    fn has_sides() -> bool {
        false
    }
}

impl CopyOfHasSides for Circle {
    fn has_sides() -> bool {
        false
    }
}

impl HasSides for Square {
    fn has_sides() -> bool {
        true
    }
}

impl HasSides for Rectangle {
    fn has_sides() -> bool {
        true
    }
}

// Static Dispatch (Compile Time)
fn trait_example(){
    println!("\nStatic Dispatch (Compile Time)");
    
    let circle = Circle { radius:5.0 };
    let square = Square { side: 4.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    circle.draw();
    println!("Circle Area: {}", circle.area());
    <Circle as Drawable>::what_is_this();

    square.draw();
    println!("Square Area: {}", square.area());
    Square::what_is_this(); 

    rectangle.draw();
    println!("Rectangle Area: {}", rectangle.area());
    <Rectangle as Drawable>::what_is_this();

    let res = <Circle as HasSides>::has_sides();
    println!("\nCircle has sides: {res}");

    let res = <Circle as CopyOfHasSides>::has_sides();
    println!("Circle has sides: {res}");

    let res = Square::has_sides();
    println!("\nSquare has sides: {res}");

    let res = Rectangle::has_sides();
    println!("Rectangle has sides: {res}"); // If dynamic dispatch then: 1) shape.has_sides() 2) HasSides::has_sides(&shape)

    println!("\nTrait Bounds"); // Trait Bounds

    print_area(&circle);
    println!("Circle & Square are equal: {}", compare(&circle, &circle));
}

// Trait Bounds
fn print_area<T: Drawable>(shape: &T) {
    println!("Area Via Trait Bounds: {}", shape.area());
}

fn compare<T: Drawable + PartialEq>(shape1: &T, shape2: &T) -> bool {
    shape1 == shape2
}

// Dynamic Dispatch: Trait Object + Polymorphism (Run Time)
trait Draw {
    fn draw_new(&self, item: i32);
    fn name(&self) -> String;
}

impl Draw for Circle {
    fn draw_new(&self, item: i32) {
        println!("\nDrawing Circle with radius: {} and item: {item}", self.radius);
    }

    fn name(&self) -> String {
        "Circle".to_string()
    }
}

impl Draw for Square {
    fn draw_new(&self, item: i32) {
        println!("\nDrawing Square with side: {} and item: {item}", self.side);
    }

    fn name(&self) -> String {
        "Square".to_string()
    }
}

impl Draw for Rectangle {
    fn draw_new(&self, item: i32) {
        println!("\nDrawing Rectangle with width: {}, height: {} and item: {item}", self.width, self.height);
    }

    fn name(&self) -> String {
        "Rectangle".to_string()
    }
}

fn dynamic_dispatch(){
    println!("\nDynamic Dispatch");

    let circle = Circle { radius: 5.0 };
    let square = Square { side: 4.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    let shapes: Vec<&dyn Draw> = vec![&circle, &square, &rectangle];
    for shape in shapes {
        shape.draw_new(23);
        println!("Shape Name: {}", shape.name());
    }
}
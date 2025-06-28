fn main(){
    println!("🏗️  Struct Examples");

    println!("\nCreating Named Field Structs");
    let debansu = create_student();
    println!("Debanasu's Details: {:?}", debansu);

    let debadutta = create_employee();
    println!("Debadutta's Details: {:?}", debadutta);

    tuple_structs();
    unit_like_structs();
    impl_example();
    calculator_example();
}

// Types Of Structs In Rust
// 1. Named Field Struct
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    grades: [u8; 5],
    phn_num: String,
    roll_num: u32,
}

// Initializing A Struct
fn create_student() -> Student {
    println!("\nCreating a Student Struct");

    let student = Student {
        name: String::from("Debansu"),
        age: 19,
        grades: [99, 99, 99, 99, 99],
        phn_num: String::from("9114472604"),
        roll_num: 1,
    };

    // Accessing Struct Fields
    println!("Name: {}, Age: {}, Grades: {:?}, Phone Number: {}, Roll Number: {}", 
             student.name, student.age, student.grades, student.phn_num, student.roll_num);
    student
}

#[derive(Debug)]
struct Employee {
    emp_id: u32,
    name: String,
    dept: String,
}

fn create_employee() -> Employee {
    println!("\nCreating an Employee Struct");

    let emp_id = 101;
    let name = String::from("Debadutta");
    let dept = String::from("Computer Science");

    let employee = Employee {
        emp_id,
        name,
        dept,
    };

    // Accessing Struct Fields
    println!("Employee ID: {}, Name: {}, Department: {}", employee.emp_id, employee.name, employee.dept);
    employee
}

// 2. Tuple Struct
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(i32, i32);

// Initializing Tuple Structs
fn tuple_structs(){
    println!("\nCreating Tuple Structs");

    let white = Color(255, 255, 255);
    let black = Color(0, 0, 0);

    println!("\nWhite Color: {:?}", white);
    println!("Red Component Of Black: {}", black.0);
    println!("Green ComponentOf Black: {:?}", black.1);
    println!("Blue Component Of Black: {:?}", black.2);

    let origin = Point(0, 0);
    let point = Point(3, 4);

    println!("\nOrigin Point: {:?}", origin);
    println!("X Coordinate Of Point: {}", point.0);
    println!("Y Coordinate Of Point: {}", point.1);
}

// 3. Unit-Like Struct
#[derive(Debug)]
struct Meter;

#[derive(Debug)]
struct Uranium;

// Initializing Unit-Like Structs
fn unit_like_structs(){
    println!("\nCreating Unit-Like Structs");

    let meter = Meter;
    let uranium = Uranium;

    println!("Unit-Like Struct Created: {:?}, {:?}", meter, uranium);
}

// 4. Impl Blocks
#[derive(Debug)]
struct BasicStudent {
    name: String,
    age: u8,
    phn_num: String,
}

impl BasicStudent {
    // Constructor Method Or Associated Function
    fn new(name: String, age: u8, phn_num: String) -> Self {
        BasicStudent {
            name,
            age,
            phn_num,
        }
    }

    // Methods
    fn display_name(&self){
        println!("Student Name: {}", self.name);
    }

    fn set_name(&mut self, name: String){
        self.name = name;
    }

    fn set_age(&self, age: u8) -> Self{
        BasicStudent {
            name: self.name.clone(), // Cloning the name to avoid ownership issues
            age,
            phn_num: self.phn_num.clone(), // Or Keeping the same phone number
        }
    }

    fn set_phn_num(self, phn_num: String) -> Self {
        Self {
            phn_num,
            ..self // Using Struct Update Syntax
        }
    }

    // self: Refers to the instance of the struct
    // &self: Refers to a reference to the instance (immutable borrow)
    // &mut self: Refers to a mutable reference to the instance (mutable borrow)

    // Self: Refers to the struct type itself, used for associated functions that do not require an instance
    // Self & BasicStudent can be used interchangeably in methods, but Self is more idiomatic for associated functions.

    // ..self: Refers to the current instance of the struct, used in methods to access or modify the instance's fields.
    // ..&self: Refers to a reference to the current instance, used in methods to access fields without taking ownership.
}

// Using Impl Blocks
fn impl_example() {
    println!("\nUsing Impl Blocks");

    let mut student = BasicStudent::new(
        String::from("Debansu"), 
        19, 
        String::from("9114472604")
    );

    student.display_name();
    println!("Name: {}, Age: {}, Phone Number: {}", student.name, student.age, student.phn_num);

    student.set_name(String::from("Debadutta")); 
    println!("\nAfter Setting Name: {:?}", student);

    let updated_student = student.set_age(20);
    println!("\nAfter Setting Age: {:?}", updated_student);
    println!("Original Student After Age Update: {:?}", student);  

    let new_student = updated_student.set_phn_num(String::from("1234567890"));
    println!("\nAfter Setting Phone Number: {:?}", new_student);
    println!("Original Student After Phone Number Update: {:?}", student); 
    // println!("Updated Student After Phone Number Update: {:?}", updated_student); // As updated_student is borrowed, so it cannot be used
}

#[derive(Debug)]
struct Calculator {
    cur_val: f64,
    history: Vec<f64>,
}

impl Calculator{
    fn new() -> Self {
        Self { 
            cur_val: 0.0, 
            history: vec![] 
        }
    }

    fn with_initial_value(val: f64) -> Self {
        Self {
            cur_val: val,
            history: vec![val],
        }
    }

    fn add(&mut self, a: f64, b: f64) -> f64 {
        let res = a + b;
        self.cur_val = res;
        self.history.push(self.cur_val);
        res
    }

    fn subtract(&mut self, a: f64, b: f64) -> f64 {
        let res = a - b;
        self.cur_val = res;
        self.history.push(self.cur_val);
        res
    }

    fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let res = a * b;
        self.cur_val = res;
        self.history.push(self.cur_val);
        res
    }

    fn divide(&mut self, a: f64, b: f64) -> f64 {
        if b == 0.0 {
            panic!("Zero Division Error!!!");
        }
        let res = a / b;
        self.cur_val = res;
        self.history.push(self.cur_val);
        res
    }

    fn clear(&mut self) {
        self.cur_val = 0.0;
        self.history.clear();
    }
}

// Example usage of the Calculator struct and its methods
fn calculator_example(){
    println!("\nSimple Calculator Example");

    let mut calc_one = Calculator::new(); // Create a new calculator with default value (0.0)
    println!("Calculator 1 Initial Condition: {:?}", calc_one);

    let mut calc_two = Calculator::with_initial_value(10.01); // Create a new calculator with an initial value
    println!("Calculator 2 Initial Condition: {:?}", calc_two);

    let sum = calc_one.add(10.00, 20.00);
    println!("\nCalculator 1: 10.00 + 20.00 = {sum}");
    println!("Current State Of Calculator 1: {:?}", calc_one);

    println!("\nCalculator 2: 10.00 / 2.00 = {}", calc_two.divide(10.00, 2.00));
    println!("Current State Of Calculator 2: {:?}", calc_two);

    println!("\nCalculator 1: 100.00 - 20.00 = {}", calc_two.subtract(100.00, 20.00));
    println!("Current State Of Calculator 1: {:?}", calc_two);

    println!("\nCalculator 2: 100.00 * 20.00 = {}", calc_two.multiply(100.00, 20.00));
    println!("Current State Of Calculator 2: {:?}", calc_two);

    calc_one.clear();
    println!("\nCurrent State Of Calculator 1: {:?}", calc_one);

    calc_two.clear();
    println!("Current State Of Calculator 2: {:?}", calc_two);
}
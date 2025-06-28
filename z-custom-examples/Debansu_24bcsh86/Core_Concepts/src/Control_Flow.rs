fn main() {
    println!("🔁 Control Flow");
    greet();
    greet_person("Debansu");

    println!("\n5 + 6 = {}", add(5, 6));
    println!("5.0 / 6.0 = {}", divide(5.0, 6.0));
    println!("5.0 / 0.0 = {}\n", 5.0 / 0.0);

    scope_example();
    other_scope_example();
    custom_scope();
    custom_scope2();

    if_example();
    if_else_expression();
    match_example();

    loop_example();
    while_example();
    for_example();
}

// Simple Function Without Input Or Output
fn greet() {
    println!("\nHello Siliconites!");
}

// Function With Parameter
fn greet_person(name: &str) {
    println!("\nHello {name}!");
}

// Function With Input Or Output
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Early Return Function
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Zero Division Error.");
        return 0.0;
    }

    a / b
}

// Scope Rules
fn scope_example() {
    let x = 10;
    println!("In this function, value of x: {x}");
}

fn other_scope_example() {
    let y = 20;
    println!("In this function, value of y: {y}");
}

fn custom_scope() {
    let a  = 100;
    {
        let b = 200;
        {
            let c = 300;
            println!("\nIn Nested Inner Scope, A = {a}, B = {b}, C = {c}");
        }
        println!("In Inner Scope, A = {a}, B = {b}");
    }
    println!("In Outer Scope, A = {a}");
}

// Shadowing & Nested Scopes
fn custom_scope2() {
    let x  = 10;
    {
        let x = 20;
        println!("\nInner Scope X = {x}");
    }
    {
        let y = 30;
        println!("Inner Scope X = {x}, Y = {y}");
    }
    println!("In Outer Scope, X = {x}");
}

// Simple if & if-else
fn if_example() {
    let num = 5;

    if num > 0 {
        println!("\nThe number is positive."); 
    } else { 
        println!("\nThe number is negative.");
    }

    if num > 0 {
        println!("{num} is positive.") // Can skip semicolon
    } else if num < 0 {
        println!("{num} is negative.")
    } else {
        println!("{num} is zero.")
    }
}

// if-else as expressions
fn if_else_expression() {
    let condition = true;
    let number = if condition {5} else {6};
    println!("\nThe value of number: {number}");

    let age = 19;
    let age_group = if age >= 18 {"Adult"} else {"Minor"};
    println!("\nThe age group is: {age_group}");

    let has_license = true;
    if age >= 18 && has_license {
        println!("You are eligible to drive.");
    } else if age >= 18 && !has_license {
        println!("You are an adult but need a license.");
    } else {
        println!("You are not eligible to drive.");
    }
}

// Match Expression
fn match_example() {
    let number = 10;

    match number {
        1 => println!("\nOne"),
        2 => println!("\nTwo"),
        3 => println!("\nThree"),
        _ => println!("\nWildcard In Action!"),
    }

    match number {
        1 | 2 | 3  => println!("One, Two or Three"),
        4 | 5 | 6 => println!("Four, Five or Six"),
        7 | 8 | 9 => println!("Seven, Eight or Nine"),
        _ => println!("Wildcard In Action!"),
    }

    match number {
        1..=10 => println!("One to Ten."),
        11..=50 => println!("Eleven to Fifty."),
        51..=100 => println!("Fifty One to One Hundred."),
        _ => println!("Wildcard In Action!"),
    }
}

// Infinite Loop With Break
fn loop_example() {
    println!("\nLoop Example:\n");

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            println!("Counter reached 10, exiting loop.");
            break;
        }
        println!("Counter: {counter}");
    }
}

// While Loop
fn while_example() {
    println!("\nWhile Loop:\n");

    let mut counter = 0;
    while counter <= 10 {
        counter += 1;
        println!("Counter: {counter}");
    }
}

// For Loop Variations
fn for_example() {
    println!("\nFor Loop:");

    println!("\nVariant 1:");
    for i in 1..=10 {
        println!("Current Number: {i}");
    }

    println!("\nVariant 2:");
    for i in (1..=10).rev() {
        println!("Current Number: {i}");
    }

    println!("\nVariant 3:");
    for i in (1..=10).rev().step_by(2) {
        println!("Current Number: {i}");
    }

    println!("\nVariant 4:");
    for i in 1..15 {
        if i % 2 == 0 {
            continue;
        }
        println!("Current Number: {i}");
    }
}
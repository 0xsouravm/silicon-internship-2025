use std::io;
use rand::Rng;

fn main() {
    println!("Holaaa folks : ");
    println!("Choose:");
    println!("1 = Rock");
    println!("2 = Paper");
    println!("3 = Scissors");

    let mut input = String::new();  //this is the mutable string to get input u will get taught later in class (yes u will)
    io::stdin().read_line(&mut input).unwrap();  //input from user similar to scanf(&blablabla) in C like u did on college
    let user: i32 = input.trim().parse().unwrap();  //parse the input to an integer, because u can put anything from keyboard i dont trust u, u will learn about error handling later chill

    let computer = rand::thread_rng().gen_range(1..=3);  //now this will generate a random number between 1 and 3 for the computer's choice, well its inbuilt in rust

    println!("You chose : {}", user);
    println!("Computer chose: {}", computer);  


    //match case u guys studied so i dont need to tell u duhhhh
    match (user, computer) {
        (x, y) if x == y => println!("Draww !!"),
        (1, 3) | (2, 1) | (3, 2) => println!("You win!"),
        (1, 2) | (2, 3) | (3, 1) => println!("You lose!"), 
        _ => println!("Invalid choice (i told u dummy) ! Please enter 1, 2, or 3."),
    }

    println!("Arigato !");  //this is the end of the game, u can play again if u want, but i dont care so aint giving u guys a loop to replay haha implement urself
}


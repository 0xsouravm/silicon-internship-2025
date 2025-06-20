fn no_dangle() -> String {
    let s = String::from("hello");
    s 
}

fn main() {
    let r = no_dangle();
    println!("{}", r);
}

use std::io;

fn main() {
    let mut a = String::new();
    println!("======Set Number======");
    io::stdin().read_line(&mut a).expect("Error Read");
    if a.trim() != "0" {
        println!("you number is not zero");
        println!("You number: {}", a.trim());
    }
    if a.trim() == "0" {
        println!("You Number IS ZERO");
    }
}


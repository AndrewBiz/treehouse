#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn get_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name
        .trim()
        .to_lowercase()
}

fn main() {
    let visitor_list = ["andrew", "anb", "slot"];
    let mut allow_them_in = false;
    println!("Hi, your name please!");
    let name = get_name();
    for visitor in &visitor_list{
        if visitor == &name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Hello, {}", name);
    } else {
        println!("Sorry, you aren't in the list");
    }

}

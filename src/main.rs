#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_ascii_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("andrew", "Hello bro!"),
        Visitor::new("anb", "Hi master!"),
        Visitor::new("fil", "Hello kitty!"),
    ];
    println!("Hi, your name please!");
    let name = get_name();
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Go away!"),
    }
}

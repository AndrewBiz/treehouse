#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
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
    let mut visitor_list = vec![
        Visitor::new("andrew", "Hello bro!"),
        Visitor::new("anb", "Hi master!"),
        Visitor::new("fil", "Hello kitty!"),
    ];

    loop {
        println!("Hi, your name please! (empty to quit)");
        let name = get_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                }
                println!("{} is not on the visitor list.", name);
                visitor_list.push(Visitor::new(&name, "New friend"));
            }             
        }
    }
    println!("The final list:");
    println!("{:#?}", visitor_list);
}

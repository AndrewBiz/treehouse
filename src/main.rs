#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a temp member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("andrew", VisitorAction::Accept, 46),
        Visitor::new(
            "anb",
            VisitorAction::AcceptWithNote {
                note: String::from("Hi, master!"),
            },
            15,
        ),
        Visitor::new("fil", VisitorAction::Refuse, 1),
    ];

    loop {
        println!("Hi, your name please! (empty to quit)");
        let name = get_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        if let Some(visitor) = known_visitor {
            visitor.greet_visitor();
        } else {
            if name.is_empty() {
                break;
            };
            println!("{} is not on the visitor list.", name);
            visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
    }
    println!("The final list:");
    println!("{:#?}", visitor_list);
}

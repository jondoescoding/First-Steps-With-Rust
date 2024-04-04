use std::io::stdin;

// An enumeration for the types of action which can be taken by a visitor
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String},
    Refuse,
    Probation,
}

// Visitor struct 
#[derive(Debug)]
struct Visitor{
    name: String,
    action: VisitorAction,
    age: i8
}

// Implementation of the Visitor struct
impl Visitor {
    fn new (name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }
    fn greet_visitor(&self){
        match &self.action {
            VisitorAction::Accept => println!("Welcome!, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to: {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{}, is on probation", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in", self.name)                                                                          
        }
    }

}

// Functions
fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    return your_name
        .trim()
        .to_lowercase()
}


fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("Steve", VisitorAction::AcceptWithNote { note: String::from("Milk is in the fridge") }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30)
    ];

    'main_loop: loop { // Labeled loop
        println!("Hello, what is your name? ");
        let name = what_is_your_name();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);
    
        match known_visitor {
                Some(visitor) => visitor.greet_visitor(),
                None => {
                    if name.is_empty(){
                        break 'main_loop;
                    } else {
                        println!("{} is not on the visitor list", name);
                        visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                    }
                }
            }

    }

}

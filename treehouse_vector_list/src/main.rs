use std::io::stdin;

#[derive(Debug)]
struct Visitor{
    name: String,
    greeting: String,
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    return your_name
        .trim()
        .to_lowercase()
}

impl Visitor {
    fn new (name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }
    fn greet_visitor(&self){
        println!("{}", self.greeting);
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", "Peace be with you"),
        Visitor::new("Steve", "Shalom, Steve"),
        Visitor::new("Fred", "Fred is the man")
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
                    visitor_list.push(Visitor::new(&name, "Welcome new friend"));
                }
            }
        }
    }

    println!("Current list of visitors: {:#?}", visitor_list)
}

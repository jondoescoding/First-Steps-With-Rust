use std::{io::stdin, process::exit};

#[derive(Debug)]
struct Person {
    name: String,
    age: i8,
    mortality: PersonMortality
}

// user-defined data type that allows us to select a value from a list of related values.
// enum values are known as variants
#[derive(Debug)]
enum PersonMortality {
    Dead,
    Alive
}

impl Person {
    fn new (name: &str, age: i8, mortality: PersonMortality) -> Self {
        Self {
            name: name.to_lowercase(),
            age,
            mortality
        }
    }
    fn introduction(&self){
        println!("Hello! My name is: {} and I am {} years old", self.name, self.age)
    }

    fn birthday(&mut self){
        self.age = self.age + 1
    }

    fn dead_or_alive(&self){
        match self.mortality{
            PersonMortality::Dead => {
                println!("This person has passed")
            }

            PersonMortality::Alive => {
                println!("This person is alive")
            }
        }
    }
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

fn accept_int_input() -> i8 {
    let mut menu_option = String::new();
    stdin()
        .read_line(&mut menu_option)
        .expect("Error reading user input");

    let num: i8 = menu_option.trim().parse().expect("Invalid input");

    return num;
}

fn search_vector<T, F>(vec: &Vec<T>, predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    vec.iter().position(predicate)
}

fn main(){
    let mut birthday_list = vec![
        Person::new("Jonathan", 24, PersonMortality::Alive),
        Person::new("Jason", 20, PersonMortality::Alive),
        Person::new("Lonique", 22, PersonMortality::Alive),
        Person::new("Fredick", 60, PersonMortality::Dead)
    ];

    println!(r#"Choose an option below
    1. Print current birthday list
    2. Enter a person's name 
    3. Update a person's birthday
    4. Exit the program
    "#);

    loop {
        println!("Enter a number: ");
        let menu = accept_int_input();
        match menu {
            1 => {
                println!("Current birthday list {:?}", &birthday_list)
            }
    
            2 => {
                let user_name = what_is_your_name();
                
                let known_person = search_vector(&birthday_list, |person| person.name == user_name);
    
                match known_person{
                    Some(index) => {
                        println!("{:?}", birthday_list[index].introduction());
                        println!("{:?}", birthday_list[index].dead_or_alive())
                    },
                    None => {
                        println!("This person is not on the list. Adding them to the list. Name: {}", user_name);
                        birthday_list.push(Person { name: (user_name), age: (accept_int_input()), mortality: (PersonMortality::Alive) })
                    }
                }
            }
    
            3 => {
                let user_name = what_is_your_name();
                
                let known_person_index = search_vector(&birthday_list, |person| person.name == user_name);
    
                match known_person_index {
                    Some(index) => birthday_list[index].birthday(),
                    None => {
                        println!("This person is not on the list. Adding them to the list. Name: {}", user_name);
                        birthday_list.push(Person::new(&user_name, accept_int_input(), PersonMortality::Alive))
                    }
                }
            }
    
            4 => {
                exit(0);
            }
    
            _ => {
                println!("ERROR: Not a menu option")
            }
        }
    }
}

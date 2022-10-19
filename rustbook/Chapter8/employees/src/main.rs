use std::{io, collections::HashMap};

enum Action {
    Add(String, String),
    List(String),
    Quit,
    Unexpected
}

fn main() {
    let mut map = HashMap::<String, Vec<String>>::new();

    loop {
        let action = next_action();

        match action {
            Action::Add(employee, department) => {
                let employees = map.entry(department).or_insert(Vec::new());
                employees.push(employee);
            },
            Action::List(department) => {
                let employees = map.get_mut(&department);
                if let Some(employees) = employees {
                    employees.sort_unstable();
                    println!("Department '{}' has employees: {:?}", department, employees);
                } else {
                    println!("Department '{}' has no employees yet", department);
                }
            },
            Action::Quit => {
                break;
            }
            Action::Unexpected => {}
        }
    }
}

fn next_action() -> Action {
    let mut input = String::new();

    println!();
    println!("What is your action?");
    println!("a $PERSON to $DEPARTMENT => adds $PERSON to a $DEPARTMENT");
    println!("l $DEPARTMENT => list a $DEPARTMENT");
    println!("q => quits the program");

    io::stdin().read_line(&mut input).expect("Unexpected input");

    input_to_action(input)
}

fn input_to_action(s: String) -> Action {
    let words: Vec<&str> = s.split_whitespace().collect();

    match words.as_slice() {
        ["a", name, department] => Action::Add(name.to_string(), department.to_string()),
        ["l", department] => Action::List(department.to_string()),
        ["q"] => Action::Quit,
        _ => Action::Unexpected
    }
}

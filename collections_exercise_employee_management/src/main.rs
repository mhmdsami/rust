use std::collections::HashMap;
use std::io;

struct Employee {
    name: String,
    deparment: String,
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let mut new_employee = String::new();

    io::stdin()
        .read_line(&mut new_employee)
        .expect("Failed to read line");

    let new_employee: Vec<&str> = new_employee.split_whitespace().collect();

    let new_employee = Employee {
        name: new_employee[1].to_string(),
        deparment: new_employee[3].to_string(),
    };
}

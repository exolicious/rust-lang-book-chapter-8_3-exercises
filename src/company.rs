use std::{collections::HashMap};
use std::str::SplitWhitespace;

pub struct Company {
    company_map: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Self {
        let mut temp : HashMap<String, Vec<String>> = HashMap::new();
        temp.insert(String::from("Finance"), Vec::new());
        temp.insert(String::from("HR"), Vec::new());
        temp.insert(String::from("Engineering"), Vec::new());
        temp.insert(String::from("Marketing"), Vec::new());
        Self {
            company_map: temp
        }
    }

    pub fn add_employee(& mut self, no_whitespaces_command_stream: & mut SplitWhitespace) {
        self.consider_add_employee(no_whitespaces_command_stream);
    }

    fn consider_add_employee(& mut self, no_whitespaces_command_stream: & mut SplitWhitespace) {
        match no_whitespaces_command_stream.next() {
            Some(name) => { 
                match no_whitespaces_command_stream.next() {
                    Some("to") => {
                        match no_whitespaces_command_stream.next() {
                            Some(department) => self.add_employee_to_department(name, department),
                            None => { println!("You need to specify which department you want to add {} to", name) }
                        }
                    }
                    _ => { println!("After the Employee's name you should use 'to'.") }
                }
            }
            None =>  { println!("Specify the name of the Employee!") }
        }
    }
    
    fn add_employee_to_department(& mut self, employee_name: &str, department: &str) {
        match self.company_map.get_mut(department) {
            Some(employees_in_that_department) => {
                employees_in_that_department.push(String::from(employee_name));
                println!("Added '{}' to department '{}'", employee_name, department);
            },
            None => { println!("Please specify a valid department. Valid departments include: 'Finance, HR, Engineering, Marketing'") }
        }
    }

    pub fn show(& mut self, no_whitespaces_command_stream: & mut SplitWhitespace) {
        self.consider_show(no_whitespaces_command_stream);
    }
    
    fn consider_show(& mut self, no_whitespaces_command_stream: & mut SplitWhitespace) {
        match no_whitespaces_command_stream.next() {
            Some("all") => self.show_all(),
            Some(possible_department) => self.show_employees_of_department(possible_department),
            None => println!("After 'show' please specify either 'all' or {{Department}}")
        }
    }

    fn show_all(&self) {
        println!("{:#?} ", self.company_map);
    }

    fn show_employees_of_department(&self, department: &str) {
        match self.company_map.get(department) {
            Some(employees_of_that_department) => println!("Employees of department '{}': {:?}", department, employees_of_that_department),
            None => println!("Noone is currently working in {}", department)
        }
        
    }
}

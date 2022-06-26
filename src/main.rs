use std::{collections::HashMap};
use std::io;

mod company;
use company::Company;

fn main() {
    let mut test_vec = vec![4,3,1,7,6,2,5];
    println!("{}", median(&mut test_vec).unwrap());
    let test_vec_2 = vec![4,4,3,3,3,2,1];
    println!("{}", mode(&test_vec_2).unwrap());
    test_vec.sort();
    
    let mut company = Company::new();


    loop {
        println!("Use 'Add {{Name}} to {{Department}}' to add People, or 'Show all/{{Department}}' to list either all Departments or Employees in the specified Department \n
        Departments: Finance, HR, Engineering, Marketing");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let mut no_whitespaces_command = command.split_whitespace();
        match no_whitespaces_command.next() {
            Some("Add") => { company.add_employee(& mut no_whitespaces_command); },
            Some("Show") => { company.show(& mut no_whitespaces_command); } 
            Some(_) => { println!("Please begin your Instruction with either 'Add' or 'Show'.") },
            None => { println!("Please enter Something.") }
        }
    }
}


fn median(vec: &mut Vec<i32>) -> Option<f32> {
    vec.sort();
    match vec.len() {
        0 => None,
        1 => Some(vec[0] as f32),
        x => {
            if x%2 == 1 {
                Some(vec[x/2] as f32)
            }
            else {
                Some((vec[x/2-1] as f32 + vec[x/2] as f32) / 2.)
            }
        }
    }
}

fn mode(vec: &Vec<i32>) -> Option<i32> {
    let mut occurences_map: HashMap<i32, usize> = HashMap::new();
    match vec.len() {
        0 => None,
        1 => Some(vec[0]),
        _ => {
            let mut most_occurences = 0;
            let mut number_with_most_occurences = vec[0];
            for value in vec {
                let count = occurences_map.entry(*value).or_insert(0);
                *count += 1;
                if *count > most_occurences {
                    most_occurences = *count;
                    number_with_most_occurences = *value;
                }
            }
            Some(number_with_most_occurences)
        }
    }
}


mod tests;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .filter(|line| !line.is_empty())// split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {

    let lines = read_lines("C:\\Users\\mathe\\RustroverProjects\\barneydb\\src\\input.bsql");

    for mut i in 0..lines.len() {
        let cur_line = lines[i].clone(); //rust is verbose in its memory strategies, which might not be bad in the long run
        if cur_line.starts_with("create table") {
            let table_name = create_table(cur_line);
            i += 1;
            let column_map = get_columns(Vec::from(&lines[1..]), i);
            let mut file = File::open(table_name).unwrap();
            for (column_name, column_type) in column_map.into_iter() {
                file.write(column_name.as_bytes());
                file.write(";".as_bytes());
            }
            file.flush();
            break;
        }
    }

}

fn get_columns(lines: Vec<String>, i: usize) -> HashMap<String, String> {

    let mut column_map: HashMap<String,String> = HashMap::new();

    let mut i = 0;

    //get columns using the current lines and the current i index
    for j in i..lines.len() {
        let cur_line = lines[j].clone();
        if (cur_line.contains("}")) {
            break;
        }
        //cur_line should be a column declaration of the form:
        // column1 datatype,
        // employee_name string(5), --make it different from oracle by using string
        // enroll_date date,
        // special_commision bool,
        // ...
        let mut tokens = cur_line.split_whitespace();
        i += 1;
        let column_name = tokens.next().unwrap();
        let column_type = tokens.next().unwrap();
        column_map.insert(column_name.to_string(),column_type.to_string());
    }
    println!("{}", i);
    column_map

}

fn create_table(cur_line: String) -> String {
    /*
    CREATE TABLE table_name (
        column1 datatype,
        column2 datatype,
        column3 datatype,
       ....
    );
     */
    let mut table_name: String = "uninitialized".to_string();
    let tokens = cur_line.split_whitespace();
    for cur_token in tokens {
        println!("{}", cur_token);
        if cur_token == "create" {
            continue;
        } else if cur_token == "table" {
            continue;
        } else if cur_token != "{" {
            //then it must be the table_name
            table_name = cur_token.to_string();
        } else {
            // should be the opening brackets
        }
    }

    if (table_name != "uninitialized") {
        File::create(table_name.clone()).unwrap();
        return table_name;
    } else {
        //throw error todo
        println!("No table name found.");
        return "error".to_string();
    }

}

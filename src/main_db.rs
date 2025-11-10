use std::fs::read_to_string;
use regex::Regex;
use crate::{change, create_table, get, impose_uniqueness, insert, remove};

pub(crate) fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .filter(|line| !line.is_empty())// split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


pub(crate) fn main_db(lines: Vec<String>) {
    for mut i in 0..lines.len() {
        let cur_line = lines[i].clone(); //rust is verbose in its memory strategies, which might not be bad in the long run
        if cur_line.starts_with("new table") { //creates a new table
            create_table::create_table(&lines, i, cur_line);
            break;
        } else if cur_line.starts_with("add new") { //adds a line to an existing table
            insert::insert_into_table(lines, i, cur_line);
            break;
        } else if cur_line.starts_with("change") { //updates one or more records given a criteria
            change::change_value_main(cur_line);
            break;
        } else if cur_line.starts_with("remove") { //removes one or more records given a criteria
            remove::remove_main(cur_line);
            break;
        } else if cur_line.starts_with("get") { //prints all records given a criteria
            get::get_main(cur_line);
            break;
        } else if cur_line.starts_with("define uniqueness") {
            //define uniqueness for employee as (name, age);
            let mut tokens = cur_line.split_whitespace();

            let mut cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "define");

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "uniqueness");

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "for");

            let table_name = tokens.next().unwrap().to_string();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "as");

            let column_set_string = tokens.next().unwrap().to_string();
            let re = Regex::new(r", ?").unwrap();
            let unique_columns: Vec<&str> = re.split(&*column_set_string).collect(); //vector of columns that compose the uniqueness criteria (primary key)

            impose_uniqueness(table_name, unique_columns);
        }
    }
}
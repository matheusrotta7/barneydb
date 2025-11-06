mod tests;
mod create_table;
mod insert;
mod canonical_columns;
mod change;
mod remove;
mod get;

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};

use std::fs::{read_to_string, File, OpenOptions};
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .filter(|line| !line.is_empty())// split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {

    let lines = read_lines("C:\\Users\\mathe\\RustroverProjects\\barneydb\\src\\sample_scripts\\get.bsql");

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

fn impose_uniqueness(table_name: String, unique_columns: Vec<&str>) {
    //open table file
    //check if the table already has a uniqueness criteria
    //if it does, report it and stop. Recommend dropping it and recreating the new criteria
    //if it doesn't, add the uniqueness criteria to the table file (must understand where to store it)
    //metadata in the table for now is just the header with the column names and types,
    //but if it grows too much, it should be stored in a separate file
    //maybe it could be a table folder, with a metadata file, and a data file
    //and if one day we want to do partitions, each partition could be a file
}





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
            //get employee if name = "Alessandra";
            get::get_main(cur_line);
            break;
        }
    }

}





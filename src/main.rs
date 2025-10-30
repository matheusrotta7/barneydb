mod tests;
mod create_table;
mod insert;

use std::io::{BufRead, Read, Write};

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

    let lines = read_lines("C:\\Users\\mathe\\RustroverProjects\\barneydb\\src\\insert.bsql");

    for mut i in 0..lines.len() {
        let cur_line = lines[i].clone(); //rust is verbose in its memory strategies, which might not be bad in the long run
        if cur_line.starts_with("new table") {
            create_table::create_table(&lines, i, cur_line);
            break;
        } else if cur_line.starts_with("add new") {
            insert::insert_into_table(lines, i, cur_line);
            break;
        } else if cur_line.starts_with("change") {
            //change table_name set column_name_updated = new_value if column_name_criteria = criteria_value;
            let mut tokens = cur_line.split_whitespace();
            let mut cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "change");

            cur_token = tokens.next().unwrap().to_string();
            let table_name = cur_token.clone();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "set");

            cur_token = tokens.next().unwrap().to_string();
            let updated_column_name = cur_token.clone();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "=");

            cur_token = tokens.next().unwrap().to_string();
            let new_column_value = cur_token.clone();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "if");

            cur_token = tokens.next().unwrap().to_string();
            let criteria_column = cur_token.clone();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "=");

            cur_token = tokens.next().unwrap().to_string();
            let criteria_value = cur_token.clone();

            todo!()
        }
    }

}




mod tests;
mod create_table;
mod insert;
mod canonical_columns;

use std::io::{BufRead, BufReader, Read, Write};

use std::fs::{read_to_string, File, OpenOptions};
use indexmap::IndexMap;

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
            let updated_column = cur_token.clone();

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

            change_value(table_name, updated_column, new_column_value, criteria_column, criteria_value);
        }
    }

}

fn change_value(table_name: String, updated_column: String, new_column_value: String, criteria_column: String, criteria_value: String) -> Result<(), Box<dyn std::error::Error>> {
    //open table file
    //identify correct index of the updated_column and the criteria_column
    //for each line:
    //    if (value in criteria_column = criteria_value)
    //         change value of updated_column to new_column_value
    //done

    let table_file = File::open("foo.txt")?;

    let reader = BufReader::new(table_file);


    let mut line_iterator = reader.lines();
    let first_line = line_iterator.next().unwrap();
    // let canonical_column_map: IndexMap<String, (String, String)>;
    let canonical_column_map = canonical_columns::get_canonical_columns(first_line.unwrap());
    let updated_column_index = canonical_column_map.get_index_of(&updated_column);
    let criteria_column_index = canonical_column_map.get_index_of(&criteria_column);


    for cur_line in line_iterator {
        let line = cur_line.unwrap();
        let mut tokens = line.split(";");
        let token_vector: Vec<String> = tokens.map(|f| f.to_string()).collect();
        if (token_vector[criteria_column_index] == criteria_value) {
            for i in 0..token_vector.len() {

            }
        }
        // println!("{}", line?);
    }

    Result::Ok(())

}




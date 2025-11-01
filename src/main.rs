mod tests;
mod create_table;
mod insert;
mod canonical_columns;
mod change;

use std::fs;
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

    let lines = read_lines("C:\\Users\\mathe\\RustroverProjects\\barneydb\\src\\sample_scripts\\remove.bsql");

    for mut i in 0..lines.len() {
        let cur_line = lines[i].clone(); //rust is verbose in its memory strategies, which might not be bad in the long run
        if cur_line.starts_with("new table") {
            create_table::create_table(&lines, i, cur_line);
            break;
        } else if cur_line.starts_with("add new") {
            insert::insert_into_table(lines, i, cur_line);
            break;
        } else if cur_line.starts_with("change") {
            change::change_value_main(cur_line);
        } else if cur_line.starts_with("remove") {
            //remove employee if name = "Josh";
            let mut tokens = cur_line.split_whitespace();
            let mut cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "remove");

            cur_token = tokens.next().unwrap().to_string();
            let table_name = cur_token.clone();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "if");

            cur_token = tokens.next().unwrap().to_string();
            let criteria_column = cur_token.clone();

            cur_token = tokens.next().unwrap().to_string();
            assert_eq!(cur_token, "=");

            cur_token = tokens.next().unwrap().to_string();
            let mut criteria_value = cur_token.clone();
            if criteria_value.ends_with(";") {
                criteria_value.pop();
            }

            remove(table_name, criteria_column, criteria_value).expect("TODO: panic message");
        }
    }

}

fn remove(table_name: String, criteria_column: String, criteria_value: String) -> Result<(), Box<dyn std::error::Error>> {
    //open table file
    //identify correct index of the criteria_column
    //for each line:
    //    if (value in criteria_column = criteria_value)
    //         don't copy line to aux file (remove it)
    //done

    let table_file = File::open(table_name.clone())?;
    let mut table_file_aux = File::create(table_name.clone() + ".aux")?;

    let reader = BufReader::new(table_file);


    let mut line_iterator = reader.lines();
    let first_line = line_iterator.next().unwrap().unwrap().to_string();
    // let canonical_column_map: IndexMap<String, (String, String)>;
    let canonical_column_map = canonical_columns::get_canonical_columns(first_line.clone());

    //write header to aux file otherwise you lose it:
    table_file_aux.write((first_line+"\n").as_bytes())?;

    let criteria_column_index = canonical_column_map.get_index_of(&criteria_column).unwrap();

    for cur_line in line_iterator {
        let line = cur_line.unwrap();
        let tokens = line.split(";");
        let token_vector: Vec<String> = tokens.map(|f| f.to_string()).collect();

        if token_vector[criteria_column_index] != criteria_value {
            table_file_aux.write((line + "\n").as_bytes())?; //if remove condition is not satisfied, you can copy the line
        } else {
            //if remove condition is satisfied, don't copy the line
            //funnily enough, in this branch you don't need to do anything
        }


    }

    //delete old table file
    fs::remove_file(table_name.clone()).expect("TODO: panic message");

    //change aux file name to table name
    fs::rename(table_name.clone() + ".aux", table_name)?;
    Result::Ok(())
}





mod tests;
mod create_table;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

use std::fs::read_to_string;
use std::ptr::null;
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
        if cur_line.starts_with("create table") {
            create_table::create_table(&lines, i, cur_line);
            break;
        } else if cur_line.starts_with("add new") {
            let table_name = get_table_name(cur_line);
            let column_map = get_column_map(lines.clone(),  i);
            insert_values_into_file(table_name, &column_map);
            break;
        }
    }

}

fn get_column_map(lines: Vec<String>, i:  usize) -> IndexMap<String, String> {
    let mut column_map: IndexMap<String, String> = IndexMap::new();

    for j in i..lines.len() {
        let cur_line = lines[j].clone();
        // add new employee {
        //     name -> "Matheus",
        //     age -> 29,
        //     date_of_birth -> instant("02/09/1996", "dd/mm/yyyy"),
        //     salary -> 200
        // }
        let mut tokens = cur_line.split_whitespace();
        let column_name = tokens.next().unwrap().to_string();
        assert_eq!(tokens.next().unwrap().to_string(), "->");
        let column_value = tokens.next().unwrap().to_string();
        column_map.insert(column_name, column_value);
    }
    column_map
}

fn get_table_name(cur_line: String) -> String {
    let mut tokens = cur_line.split_whitespace();
    let mut cur_token = tokens.next().unwrap();
    let mut table_name = "uninitialized".to_string();
    assert_eq!(cur_token, "add");
    cur_token = tokens.next().unwrap();
    assert_eq!(cur_token, "new");
    table_name = tokens.next().unwrap().to_string();
    table_name
}

fn insert_values_into_file(table_name: String, columnMap: &IndexMap<String, String>) {
    //open file
    //go to last line
    //create new line
    //for each entry in column map, locate the correct column and insert the correct value
    //it's easier if it's in the canonical order
    //maybe order by canonical order first and then insert
    todo!()
}


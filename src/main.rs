mod tests;
mod create_table;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Write};

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
            let column_map = get_column_map(lines.clone(),  i+1);
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
        if cur_line.contains("}") {
            break;
        }
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

fn insert_values_into_file(table_name: String, valuesMap: &IndexMap<String, String>) {
    //open file
    //go to last line
    //create new line
    //for each entry in column map, locate the correct column and insert the correct value
    //it's easier if it's in the canonical order
    //maybe order by canonical order first and then insert
    let lines = read_lines(&*table_name);
    let first_line = lines[0].clone();
    let mut canonical_column_map: IndexMap<String, (String, String)> = IndexMap::new();
    canonical_column_map = get_canonical_columns(first_line);
    for (column_name, column_type) in canonical_column_map {
        valuesMap[column_name];
    }
}

fn get_canonical_columns(table_header: String) -> IndexMap<String, (String, String)> {

    let mut result: IndexMap<String, (String,String)> = IndexMap::new();

    let columns = table_header.split(";").clone();

    for cur_column in columns {
        let mut tokens = cur_column.split_whitespace();
        if (cur_column == " ") {
            break;
        }

        let column_name = tokens.next().unwrap().to_string();
        let column_type = tokens.next().unwrap().to_string();
        let mut column_quantifier = "uninitialized".to_string();
        if let Some(quantifier_token) = tokens.next() {
            // safe: you still have the token
            column_quantifier = quantifier_token.to_string()
        }

        result.insert(column_name, (column_type, column_quantifier));
    }

    result
}


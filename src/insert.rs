use std::io::Write;
use std::fs::OpenOptions;
use indexmap::IndexMap;
use regex::Regex;
use crate::{canonical_columns, main_db};

pub(crate) fn insert_into_table(lines: Vec<String>, mut i: usize, cur_line: String) {
    let table_name = get_table_name(cur_line);
    let column_map = get_column_map(lines.clone(), i + 1);
    insert_values_into_file(table_name, &column_map);
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
    let lines = main_db::read_lines(&*table_name);
    let first_line = lines[0].clone();
    let canonical_column_map  = canonical_columns::get_canonical_columns(first_line);

    let mut column_values: Vec<String> = Vec::new();
    for (column_name, column_type) in canonical_column_map {
        let option = valuesMap.get(&column_name);
        let mut column_value_string = option.unwrap().to_string();
        if column_value_string.ends_with(',') {
            column_value_string.pop();
        }
        println!("{}", column_value_string);

        if (!value_respects_type_constraints(column_type.clone(), column_value_string.clone())) {
            println!("{} with value {} didn't respect type {}", column_name, column_value_string, column_type.0 + column_type.1.as_str());
            panic!("crash and burn");
        }
        column_values.push(column_value_string);
    }

    insert_values_in_bottom_of_file(column_values, table_name).expect("TODO: panic message");
}

fn value_respects_type_constraints(column_type: (String, String), column_value_string: String) -> bool {
    if column_type.0 == "string" {
        //then expect "sample_string" and check if it's not bigger than max size
        let pattern = r#""(([A-Za-z0-9]|_)+)""#;
        let re = Regex::new(pattern).unwrap();



        if re.is_match(column_value_string.as_str()) {
            println!("The string matches the pattern!");
            let max_string_size = column_type.1.parse().unwrap();
            column_value_string.len() <= max_string_size
        } else {
            println!("No match.");
            false
        }

    } else if column_type.0 == "int" {
        //then expect 3456 and check if it's not bigger than max size
        let pattern = r"\d+";
        let re = Regex::new(pattern).unwrap();

        if re.is_match(column_value_string.as_str()) {
            println!("The integer matches the pattern!");
            let max_int_size = column_type.1.parse().unwrap();
            column_value_string.len() <= max_int_size
        } else {
            println!("No match.");
            false
        }
    } else if column_type.0 == "instant" {
        let pattern = r#"instant\("\d\d/\d\d/\d\d\d\d","dd/mm/yyyy"\)"#;
        let re = Regex::new(pattern).unwrap();

        if re.is_match(column_value_string.as_str()) {
            println!("The instant matches the pattern!");
            true
        } else {
            println!("No match.");
            false
        }
    } else {
        panic!("Unexpected column type: {}", column_type.0);
    }
}

fn insert_values_in_bottom_of_file(column_values: Vec<String>, table_name: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true) // open in append mode
        .create(true) // create file if it doesn’t exist
        .open(table_name)?;

    let new_line = format!("{}", column_values.join(";"));

    writeln!(file, "{}", new_line.to_string())?;
    Ok(())
}


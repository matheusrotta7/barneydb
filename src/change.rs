use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use crate::canonical_columns;

pub(crate) fn change_value_main(cur_line: String) {
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
    let mut criteria_value = cur_token.clone();
    if criteria_value.ends_with(";") {
        criteria_value.pop();
    }

    change_value(table_name, updated_column, new_column_value, criteria_column, criteria_value);
}

fn change_value(table_name: String, updated_column: String, new_column_value: String, criteria_column: String, criteria_value: String) -> Result<(), Box<dyn std::error::Error>> {
    //open table file
    //identify correct index of the updated_column and the criteria_column
    //for each line:
    //    if (value in criteria_column = criteria_value)
    //         change value of updated_column to new_column_value
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

    let updated_column_index = canonical_column_map.get_index_of(&updated_column).unwrap();
    let criteria_column_index = canonical_column_map.get_index_of(&criteria_column).unwrap();


    for cur_line in line_iterator {
        let line = cur_line.unwrap();
        let mut tokens = line.split(";");
        let token_vector: Vec<String> = tokens.map(|f| f.to_string()).collect();

        if token_vector[criteria_column_index] != criteria_value {
            table_file_aux.write(line.as_bytes())?; //if update condition is not satisfied, you can just copy the whole line
            continue; //skip rest of loop
        }

        for i in 0..token_vector.len() {
            if i == updated_column_index {
                if token_vector[criteria_column_index] == criteria_value {
                    table_file_aux.write(new_column_value.as_bytes())?; //if update condition is satisfied, use new value
                } else {
                    table_file_aux.write(token_vector[i].as_bytes())?;   //if not, use existing value
                }
            } else {
                table_file_aux.write(token_vector[i].as_bytes())?;
            }
            if i != token_vector.len() - 1 { //don't add a semicolon in the last iteration
                table_file_aux.write(b";")?;
            }
        }
        table_file_aux.write(b"\n")?; //skip to next line

        // println!("{}", line?);
    }

    //delete old table file
    fs::remove_file(table_name.clone()).expect("TODO: panic message");

    //change aux file name to table name
    fs::rename(table_name.clone() + ".aux", table_name)?;
    Result::Ok(())

}

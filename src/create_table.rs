use std::fs::File;
use std::io::Write;
use indexmap::IndexMap;

pub(crate) fn create_table(lines: &Vec<String>, mut i: usize, cur_line: String) {
    let table_name = create_table_file(cur_line);
    i += 1;
    let column_map = get_columns(Vec::from(&lines[1..]), i);
    let mut file = File::create(table_name).unwrap();
    for (column_name, column_type) in column_map.into_iter() {
        let (type_name, type_size) = column_type;
        println!("Column {}: {} ({})", column_name, type_name, type_size);
        // write to file safely
        write!(file, "{} {} ; ", column_name, format!("{} {}", type_name, type_size)).unwrap();
    }
    file.flush();
}

fn get_columns(lines: Vec<String>, i: usize) -> IndexMap<String, (String, String)> {

    let mut column_map: IndexMap<String, (String, String)> = IndexMap::new();

    let mut i = 0;

    //get columns using the current lines and the current i index
    for j in i..lines.len() {
        let cur_line = lines[j].clone();
        if cur_line.contains("}") {
            break;
        }
        //cur_line should be a column declaration of the form:
        // column1 datatype,
        // employee_name string 5, --make it different from oracle by using string
        // enroll_date instant,
        // special_commision bool,
        // ...
        let mut tokens = cur_line.split_whitespace();
        i += 1;
        let mut column_name = tokens.next().unwrap().to_string();
        if column_name.ends_with(":") {
            column_name.pop();
        }

        let mut column_type = tokens.next().unwrap().to_string();
        if (column_type.ends_with(",")) {
            column_type.pop();
        }

        let mut column_quantifier = "".to_string();
        if let Some(quantifier_token) = tokens.next() {
            // safe: you still have the token
            column_quantifier = quantifier_token.to_string()
        }
        if (column_quantifier.ends_with(",")) {
            column_quantifier.pop();
        }
        column_map.insert(column_name,(column_type,column_quantifier.clone()));
    }
    println!("{}", i);
    column_map

}

fn create_table_file(cur_line: String) -> String {
    /*
    CREATE TABLE table_name (
        column1 datatype,
        column2 datatype,
        column3 datatype,
       ....
    );
     */
    let mut table_name: String = "uninitialized".to_string();
    let tokens = cur_line.split_whitespace();
    for cur_token in tokens {
        println!("{}", cur_token);
        if cur_token == "create" {
            continue;
        } else if cur_token == "table" {
            continue;
        } else if cur_token != "{" {
            //then it must be the table_name
            table_name = cur_token.to_string();
        } else {
            // should be the opening brackets
        }
    }

    if (table_name != "uninitialized") {
        File::create(table_name.clone()).unwrap();
        table_name
    } else {
        //throw error todo
        println!("No table name found.");
        "error".to_string()
    }

}

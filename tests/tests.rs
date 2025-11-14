
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};



    #[test]
    fn test_create_table() -> Result<(), Box<dyn std::error::Error>> {
        let lines = main_db::read_lines("C:\\Users\\mathe\\RustroverProjects\\barneydb\\src\\sample_scripts\\create_table.bsql");
        main_db::main_db(lines);

        let table_name = "employee".to_string();

        let table_file = File::open(table_name.clone())?;

        let reader = BufReader::new(table_file);


        let mut line_iterator = reader.lines();
        let first_line = line_iterator.next().unwrap().unwrap().to_string();

        // check if employee table has the correct columns
        assert!(first_line.contains("name string 100 ; age int 5 ; date_of_birth instant  ; salary int 10 ;")); // example

        Ok(())
    }

    
}
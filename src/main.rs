mod create_table;
mod insert;
mod canonical_columns;
mod change;
mod remove;
mod get;
mod main_db;


fn main() {

    let lines = main_db::read_lines("C:\\Users\\mathe\\RustroverProjects\\barneydb\\src\\sample_scripts\\get.bsql");

    main_db::main_db(lines);

}



fn impose_uniqueness(table_name: String, unique_columns: Vec<&str>) {
    //open table file
    //check if the table already has a uniqueness criteria
    //if it does, report it and stop. Recommend dropping it and recreating the new criteria
    //if it doesn't, add the uniqueness criteria to the table file (must understand where to store it)
    //metadata in the table for now is just the header with the column names and types,
    //but if it grows too much, it should be stored in a separate file
    //maybe it could be a table folder, with a metadata file, and a data file
    //and if one day we want to do partitions, each partition could be a file
}





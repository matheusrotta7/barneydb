use indexmap::IndexMap;

pub(crate) fn get_canonical_columns(table_header: String) -> IndexMap<String, (String, String)> {

    let mut result: IndexMap<String, (String,String)> = IndexMap::new();

    let columns = table_header.split(";").clone();

    for cur_column in columns {
        let mut tokens = cur_column.split_whitespace();
        if cur_column == " " || cur_column.is_empty() {
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
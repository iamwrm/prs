use anyhow::Result;

pub fn print_query_result(connection: &sqlite::Connection, query: &str) -> Result<()> {
    println!("Query: {}", query);
    println!("----------------");

    let mut stmt = connection.prepare(query)?;
    let mut cursor = stmt.iter();

    print_cursor(&mut cursor)?;
    Ok(())
}

fn print_cursor(cursor: &mut sqlite::Cursor) -> Result<()> {
    let column_names: Vec<String> = cursor
        .column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("{}", column_names.join("\t"));

    for row_result in cursor.by_ref() {
        match row_result {
            Ok(row) => {
                let values: Vec<String> = (0..column_names.len())
                    .map(|i| format_value(&row[i]))
                    .collect();
                println!("{}", values.join("\t"));
            }
            Err(e) => {
                tracing::warn!("Error reading row: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn format_value(value: &sqlite::Value) -> String {
    match value {
        sqlite::Value::Null => "NULL".to_string(),
        sqlite::Value::Integer(v) => v.to_string(),
        sqlite::Value::Float(v) => v.to_string(),
        sqlite::Value::String(v) => v.clone(),
        sqlite::Value::Binary(v) => format!("{:?}", v),
    }
}

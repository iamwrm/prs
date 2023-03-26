use anyhow::Result;

pub fn print_query_result(connection: &sqlite::Connection, query: &str) -> Result<()> {
    let output_header = format!("Query: {}", query);
    println!("{}", output_header);
    println!("----------------");

    let mut stmt = connection.prepare(query)?;

    let mut cursor = stmt.iter();

    print_cursor(&mut cursor)?;

    Ok(())
}

struct Value {
    value: sqlite::Value,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            sqlite::Value::Null => write!(f, "NULL"),
            sqlite::Value::Integer(v) => write!(f, "{}", v),
            sqlite::Value::Float(v) => write!(f, "{}", v),
            sqlite::Value::String(v) => write!(f, "{}", v),
            sqlite::Value::Binary(v) => write!(f, "{:?}", v),
        }
    }
}

impl From<sqlite::Value> for Value {
    fn from(value: sqlite::Value) -> Self {
        Value { value }
    }
}

fn print_cursor(cursor: &mut sqlite::Cursor) -> Result<()> {
    let column_names = cursor.column_names();

    let n = column_names.len();

    println!("{}", column_names.join(", "));

    for i in cursor {
        let i = i.unwrap();
        for j in 0..n {
            let name: Value = i[j].clone().into();
            print!("{}", name);
            if j == n - 1 {
                println!();
            } else {
                print!(", ");
            }
        }
    }

    Ok(())
}

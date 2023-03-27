use anyhow::Result;

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

pub fn print_query_result(connection: &sqlite::Connection, query: &str) -> Result<()> {
    let output_header = format!("Query: {}", query);
    println!("{}", output_header);
    println!("----------------");

    let mut stmt = connection.prepare(query)?;

    let mut cursor = stmt.iter();

    print_cursor(&mut cursor)?;

    Ok(())
}

fn print_cursor(cursor: &mut sqlite::Cursor) -> Result<()> {
    let column_names = cursor.column_names();

    let n = column_names.len();

    let header = column_names.join("\t");

    let lines = cursor
        .into_iter()
        .filter(|f| {
            if f.is_err() {
                tracing::warn!("Error while iterating over cursor");
                return false;
            }
            true
        })
        .map(|i| {
            let i = i.unwrap();
            (0..n)
                .map(|j| {
                    let v: Value = i[j].clone().into();
                    format!("{}", v)
                })
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect::<Vec<String>>()
        .join("\n");
    
    println!("{}", header);
    println!("{}", lines);

    Ok(())
}

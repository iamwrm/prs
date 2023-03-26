use anyhow::Result;

pub fn print_schema(connection: &sqlite::Connection) -> Result<()> {
    let query = "PRAGMA table_info(processes);";
    print_rows(connection, query)?;

    Ok(())
}

pub fn print_rows(connection: &sqlite::Connection, query: &str) -> Result<()> {
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
    let a = cursor.column_names();

    let n = a.len();

    println!("{}", a.join(", "));

    for i in cursor {
        let i = i.unwrap();
        for j in 0..n {
            let name = i[j].clone();
            let name: Value = name.into();
            print!("{}, ", name);
        }
        println!("");
    }

    Ok(())
}

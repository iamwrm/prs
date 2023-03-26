mod output;
mod proc;

use anyhow::Result;

use output::{print_rows, print_schema};
use proc::insert_process;

fn main() -> Result<()> {
    let connection = sqlite::open(":memory:").unwrap();

    insert_process(&connection)?;

    print_schema(&connection)?;

    let query = "SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10";

    print_rows(&connection, query)?;

    Ok(())
}

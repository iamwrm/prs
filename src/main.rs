mod output;
mod proc;

use anyhow::Result;
use clap::Parser;

use output::print_query_result;
use proc::insert_process;

/// ps, with sql support
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// custom sql query
    #[arg(short, long)]
    sql: Option<String>,

    #[arg(short, long)]
    preset: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sql_query = args.sql;
    let preset = args.preset;

    let mut query = "SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10".to_string();

    if preset.is_some() && sql_query.is_some() {
        anyhow::bail!("cannot use both preset and sql query");
    }
    if preset.is_none() && sql_query.is_none() {
        println!("using default query: {}", query);
    }

    match sql_query {
        Some(q) => query = q.to_string(),
        None => {}
    }

    match preset {
        Some(p) => match p.as_str() {
            "schema" => query = "PRAGMA table_info(processes);".to_string(),
            "top10_mem" => {
                query = "SELECT * FROM processes ORDER BY vmrss ASC LIMIT 10".to_string()
            }
            _ => anyhow::bail!("unknown preset"),
        },
        None => {}
    }

    let connection = sqlite::open(":memory:").unwrap();

    insert_process(&connection)?;

    // print_schema(&connection)?;

    print_query_result(&connection, &query)?;

    Ok(())
}

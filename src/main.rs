mod output;
mod proc;

use anyhow::{Ok, Result};
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

    /// preset queries
    #[arg(short, long, value_enum)]
    preset: Option<Preset>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Preset {
    Schema,
    Top10Mem,
    TopUserMem,
}

fn get_query(sql_query: Option<String>, preset: Option<Preset>) -> Result<String> {
    let mut query = "SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10".to_string();

    if preset.is_some() && sql_query.is_some() {
        anyhow::bail!("cannot use both preset and sql query");
    }
    if preset.is_none() && sql_query.is_none() {
        println!("using default query: {}", query);
    }

    if let Some(q) = sql_query {
        query = q
    }

    if let Some(p) = preset {
        match p {
            Preset::Schema => query = "PRAGMA table_info(processes);".to_string(),
            Preset::Top10Mem => {
                query = "SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10".to_string()
            }
            Preset::TopUserMem => {
                query =
                    "SELECT user, SUM(vmrss) FROM processes GROUP BY user ORDER BY SUM(vmrss) DESC"
                        .to_string()
            }
        }
    }
    Ok(query)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sql_query = args.sql;
    let preset = args.preset;

    let query = get_query(sql_query, preset)?;

    let connection = sqlite::open(":memory:").unwrap();

    insert_process(&connection)?;

    print_query_result(&connection, &query)?;

    Ok(())
}

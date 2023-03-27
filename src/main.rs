mod cli;
mod output;
mod proc;

use anyhow::{Ok, Result};
use clap::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use cli::Preset;
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

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();

    let query = cli::get_query(args.sql, args.preset)?;

    let connection = sqlite::open(":memory:")?;

    insert_process(&connection)?;

    print_query_result(&connection, &query)?;

    Ok(())
}

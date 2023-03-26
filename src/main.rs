use anyhow::Result;
use rusqlite::Connection;

use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::*;

#[derive(Debug, Serialize, Deserialize)]
struct ProcessRecord {
    stat: procfs::process::Stat,
    status: procfs::process::Status,
}

fn main() -> Result<()> {
    println!("Hello, world!");
    // get all processes using procfs
    // this didn't include other threads, only the main threads
    let it = procfs::process::all_processes()?;
    let processes = it.map(|p| p.unwrap()).collect::<Vec<_>>();

    println!("pids: {:?}", processes);

    let connection = rusqlite::Connection::open_in_memory()?;

    // get all processes's res memory usage
    for p in processes {
        let pid = p.pid;
        let stat = p.stat().unwrap();
        let status = p.status().unwrap();
        println!("pid: {}, stat: {:?}, \nstatus: {:?} ", pid, stat, status);
    }

    Ok(())
}

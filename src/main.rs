use anyhow::Result;
use rusqlite::Connection;

use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::*;

#[derive(Debug, Serialize, Deserialize)]
struct ProcessRecord {
    pid: i32,
    name: String,
    uid: u32,
    vmrss: Option<u64>,
}

impl ProcessRecord {
    fn try_new(
        pid: i32,
        stat: Option<procfs::process::Stat>,
        status: Option<procfs::process::Status>,
    ) -> Result<Self> {
        if status.is_none() || stat.is_none() {
            anyhow::bail!("status or stat is none")
        }
        let stat = stat.unwrap();
        let status = status.unwrap();

        Ok(Self {
            pid,
            name: status.name,
            uid: status.egid,
            vmrss: status.vmrss,
        })
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    // get all processes using procfs
    // this didn't include other threads, only the main threads
    let it = procfs::process::all_processes()?;
    let processes = it.map(|p| p.unwrap()).collect::<Vec<_>>();

    let connection = rusqlite::Connection::open_in_memory()?;

    // get all processes's res memory usage
    for p in processes {
        let pid = p.pid;
        let stat = p.stat().ok();
        let status = p.status().ok();

        if let Ok(record) = ProcessRecord::try_new(pid, stat, status) {
            println!("record: {:?}", record);
            // insert into sqlite
            let parmas = to_params_named(&record)?;
        }
    }

    Ok(())
}

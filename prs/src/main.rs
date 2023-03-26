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

    connection
        .execute(
            "CREATE TABLE processes (pid INT, name TEXT, uid INT, vmrss INT)",
            [],
        )
        .unwrap();

    insert_process(&connection, processes)?;

    // get all processes's res memory usage
    query_map(&connection)?;

    println!("raw query:");
    raw_query(&connection)?;

    Ok(())
}

fn insert_process(conn: &Connection, processes: Vec<procfs::process::Process>) -> Result<()> {
    // get all processes's res memory usage
    for p in processes {
        let pid = p.pid;
        let stat = p.stat().ok();
        let status = p.status().ok();

        if let Ok(record) = ProcessRecord::try_new(pid, stat, status) {
            // insert into sqlite
            let params = to_params_named(&record)?;
            conn.execute(
                "INSERT into processes (pid, name, uid, vmrss ) VALUES (:pid, :name, :uid, :vmrss)",
                params.to_slice().as_slice(),
            )
            .unwrap();
        }
    }
    Ok(())
}

/// This function works, but I want let the user input their custom query
fn query_map(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10")?;

    let query_out = stmt.query_map([], |row| {
        Ok(ProcessRecord {
            pid: row.get(0)?,
            name: row.get(1)?,
            uid: row.get(2)?,
            vmrss: row.get(3)?,
        })
    })?;

    query_out.for_each(|p| {
        println!("{:?}", p.unwrap());
    });
    Ok(())
}

fn raw_query(conn: &Connection) -> Result<()> {
    let query = "SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10";
    let mut stmt = conn.prepare(query)?;
    let query_out = stmt.raw_query();

    Ok(())
}

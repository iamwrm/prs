use anyhow::Result;
#[derive(Debug)]
pub struct ProcessRecord {
    pid: i32,
    name: String,
    uid: u32,
    vmrss: Option<u64>,
}

impl ProcessRecord {
    pub fn try_new(
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

pub fn insert_process(connection: &sqlite::Connection) -> Result<()> {
    // get all processes using procfs
    // this didn't include other threads, only the main threads
    let it = procfs::process::all_processes()?;
    let processes = it.map(|p| p.unwrap()).collect::<Vec<_>>();
    connection.execute("CREATE TABLE processes (pid INT, name TEXT, uid INT, vmrss INT)")?;

    for p in processes {
        let pid = p.pid();
        let stat = p.stat().ok();
        let status = p.status().ok();
        let record = ProcessRecord::try_new(pid, stat, status);
        if record.is_err() {
            continue;
        }
        let record = record.unwrap();
        let query = format!(
            "INSERT INTO processes (pid, name, uid, vmrss) VALUES ({}, '{}', {}, {})",
            record.pid,
            record.name,
            record.uid,
            record.vmrss.unwrap_or(0)
        );
        connection.execute(query)?;
    }

    Ok(())
}

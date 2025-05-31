use anyhow::Result;
use cached::proc_macro::cached;

#[derive(Debug)]
pub struct ProcessRecord {
    pid: i64,
    name: String,
    uid: i64,
    vmrss_kb: i64,
    unique_kb: i64,
    user: String,
    num_threads: i64,
    cmdline: String,
}

impl ProcessRecord {
    pub fn try_new(p: procfs::process::Process) -> Result<Self> {
        let pid = p.pid();
        let status = p.status()?;
        let stat = p.stat()?;
        let statm = p.statm().ok();

        let cmdline = p.cmdline().ok().map(|c| c.join(" ")).unwrap_or_default();

        // Calculate unique memory by subtracting shared pages
        let unique_kb = statm
            .map(|s| (s.resident.saturating_sub(s.shared)) * 4)
            .unwrap_or(0);

        Ok(Self {
            pid: pid as i64,
            name: status.name,
            uid: status.egid as i64,
            vmrss_kb: status.vmrss.unwrap_or(0) as i64,
            unique_kb: unique_kb as i64,
            user: uid_to_user(status.egid),
            num_threads: stat.num_threads,
            cmdline,
        })
    }
}

#[cached]
fn uid_to_user(uid: u32) -> String {
    duct::cmd!("id", "-nu", uid.to_string())
        .stderr_capture()
        .read()
        .unwrap_or_else(|_| uid.to_string())
}

pub fn insert_process(connection: &sqlite::Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE processes (
            pid INTEGER, 
            name TEXT, 
            uid INTEGER, 
            vmrss_kb INTEGER, 
            unique_kb INTEGER, 
            user TEXT, 
            num_threads INTEGER, 
            cmdline TEXT
        )",
    )?;

    let processes: Vec<ProcessRecord> = procfs::process::all_processes()?
        .filter_map(|p| p.ok())
        .filter_map(|p| ProcessRecord::try_new(p).ok())
        .collect();

    for process in processes {
        let mut stmt = connection.prepare(
            "INSERT INTO processes (pid, name, uid, vmrss_kb, unique_kb, user, num_threads, cmdline) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )?;

        if let Err(e) = stmt
            .bind((1, process.pid))
            .and_then(|_| stmt.bind((2, process.name.as_str())))
            .and_then(|_| stmt.bind((3, process.uid)))
            .and_then(|_| stmt.bind((4, process.vmrss_kb)))
            .and_then(|_| stmt.bind((5, process.unique_kb)))
            .and_then(|_| stmt.bind((6, process.user.as_str())))
            .and_then(|_| stmt.bind((7, process.num_threads)))
            .and_then(|_| stmt.bind((8, process.cmdline.as_str())))
            .and_then(|_| stmt.next().map(|_| ()))
        {
            tracing::warn!("Failed to insert process {}: {}", process.pid, e);
        }
    }

    Ok(())
}

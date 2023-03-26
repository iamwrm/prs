use anyhow::Result;
use cached::proc_macro::cached;

#[derive(Debug)]
pub struct ProcessRecord {
    pid: i32,
    name: String,
    uid: u32,
    vmrss: Option<u64>,
    user: String,
    num_threads: i64,
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
            user: uid_to_user(status.egid),
            num_threads: stat.num_threads,
        })
    }
}

#[cached]
fn uid_to_user(uid: u32) -> String {
    uid_to_user_inner(uid).unwrap_or_else(|_| "unknown".to_string())
}

fn uid_to_user_inner(uid: u32) -> Result<String> {
    let cmd_to_run = duct::cmd!("id", "-nu", uid.to_string());
    let output = cmd_to_run.stderr_capture().read()?;
    Ok(output)
}

pub fn insert_process(connection: &sqlite::Connection) -> Result<()> {
    connection
        .execute("CREATE TABLE processes (pid INT, name TEXT, uid INT, vmrss INT, user TEXT, num_threads INT)")?;

    // get all processes using procfs
    // this didn't include other threads, only the main threads
    let processes = procfs::process::all_processes()?
        .map(|p| p.unwrap())
        .collect::<Vec<_>>();

    for p in processes {
        match ProcessRecord::try_new(p.pid(), p.stat().ok(), p.status().ok()) {
            Ok(record) => {
                let query = format!(
                    "INSERT INTO processes (pid, name, uid, vmrss, user, num_threads) VALUES ({}, '{}', {}, {}, '{}', {})",
                    record.pid,
                    record.name,
                    record.uid,
                    record.vmrss.unwrap_or(0),
                    record.user,
                    record.num_threads
                );
                connection.execute(query)?;
            }
            Err(_) => continue,
        }
    }

    Ok(())
}

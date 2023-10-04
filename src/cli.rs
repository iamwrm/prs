use anyhow::{Ok, Result};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Preset {
    /// show the schema
    Schema,
    /// show the top 10 memory processes
    Top10Mem,
    /// show the top memory users
    TopUserMem,
    /// show the top 10 memory processes of user root
    TopRootMem,
}

pub fn get_query(sql_query: Option<String>, preset: Option<Preset>) -> Result<String> {
    if preset.is_some() && sql_query.is_some() {
        anyhow::bail!("Cannot use both preset and sql query");
    }

    if preset.is_none() && sql_query.is_none() {
        let query = "SELECT * FROM processes ORDER BY vmrss_kb DESC LIMIT 10".to_string();
        tracing::info!("Using default query: {}", query);
        return Ok(query);
    }

    if let Some(q) = sql_query {
        return Ok(q);
    }

    if let Some(p) = preset {
        let query = match p {
            Preset::Schema => "PRAGMA table_info(processes);",
            Preset::Top10Mem => "SELECT * FROM processes ORDER BY vmrss_kb DESC LIMIT 10",
            Preset::TopUserMem => {
                "SELECT user, SUM(vmrss_kb) FROM processes GROUP BY user ORDER BY SUM(vmrss_kb) DESC"
            }
            Preset::TopRootMem => {
                "SELECT * FROM processes WHERE user=\'root\' ORDER BY vmrss_kb DESC LIMIT 10"
            }
        };
        tracing::info!("Using preset query: {}", query);
        Ok(query.to_string())
    } else {
        anyhow::bail!("No query found");
    }
}

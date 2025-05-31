use anyhow::Result;

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
    match (sql_query, preset) {
        (Some(_), Some(_)) => anyhow::bail!("Cannot use both preset and sql query"),
        (Some(query), None) => Ok(query),
        (None, Some(preset)) => Ok(get_preset_query(preset)),
        (None, None) => {
            let query = "SELECT * FROM processes ORDER BY vmrss_kb DESC LIMIT 10";
            tracing::info!("Using default query: {}", query);
            Ok(query.to_string())
        }
    }
}

fn get_preset_query(preset: Preset) -> String {
    let query = match preset {
        Preset::Schema => "PRAGMA table_info(processes);",
        Preset::Top10Mem => "SELECT * FROM processes ORDER BY vmrss_kb DESC LIMIT 10",
        Preset::TopUserMem => {
            "SELECT user, SUM(vmrss_kb) FROM processes GROUP BY user ORDER BY SUM(vmrss_kb) DESC"
        }
        Preset::TopRootMem => {
            "SELECT * FROM processes WHERE user='root' ORDER BY vmrss_kb DESC LIMIT 10"
        }
    };
    tracing::info!("Using preset query: {}", query);
    query.to_string()
}

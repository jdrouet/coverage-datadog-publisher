use std::{path::{PathBuf}, time::SystemTime};
use clap::Parser;

mod entity;

/// Parse and export your code coverage results to datadog
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of the project
    #[arg(long)]
    project_name: Option<String>,
    // Version of the project
    #[arg(long)]
    project_version: Option<String>,
    // Hash of the current commit
    #[arg(long)]
    commit_hash: Option<String>,
    // Name of the current branch
    #[arg(long)]
    branch_name: Option<String>,

    // Datadog site to connect to
    #[arg(long, default_value = "https://api.datadoghq.com")]
    datadog_site: String,
    // Datadog api key to authenticate
    #[arg(long)]
    datadog_api_key: String,
    
    // Base name of the series
    #[arg(long, default_value = "coverage")]
    series_name: String,
    /// Path to the json coverage report
    #[arg()]
    report: PathBuf,
}

impl Args {
    fn tags(&self) -> Vec<String> {
        let mut res = Vec::new();
        if let Some(ref value) = self.project_name {
            res.push(format!("project_name:{value}"));
        }
        if let Some(ref value) = self.project_version {
            res.push(format!("project_version:{value}"));
        }
        if let Some(ref value) = self.commit_hash {
            res.push(format!("commit_hash:{value}"));
        }
        if let Some(ref value) = self.branch_name {
            res.push(format!("branch_name:{value}"));
        }
        res
    }

    fn client(&self) -> datadog_client::client::Client {
        datadog_client::client::Client::new(self.datadog_site.clone(), self.datadog_api_key.clone())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    // reading file content
    let content = std::fs::read_to_string(&args.report).expect("unable to read report file");
    let content: entity::FileContent = serde_json::from_str(content.as_str()).expect("invalid report format");
    // building datadog client
    let client = args.client();
    // building metrics
    let tags = args.tags();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("couldn't get timestamp").as_secs();
    let metrics = content.metrics(now, args.series_name);
    let metrics: Vec<_> = metrics.into_iter().map(|m| m.set_tags(tags.clone())).collect();
    // sending to datadog
    client.post_metrics(&metrics).await.expect("unable to post metrics");
    Ok(())
}
use clap::Parser;
use std::env;
use std::error::Error;
mod jira;
/// Simple JIRA client to add worklog time to an issue
#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    /// key of jira issue
    #[clap(short, long)]
    issue: String,

    /// time (in the form: 1h, 1d, 30m, etc..)
    #[clap(short, long)]
    time: String,

    /// comment (inside " ")
    #[clap(short, long, default_value = "")]
    comment: String,

    /// started (ex: 2021-01-17T12:34:00.000+0000)
    #[clap(short, long, default_value = "")]
    started: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let jira_user = env::var("JIRA_USER").expect("Please set $JIRA_USER environnement variable");
    let jira_password =
        env::var("JIRA_PASSWORD").expect("Please set $JIRA_PASSWORD environnement variable");
    let jira_url = env::var("JIRA_URL").expect("Please set $JIRA_URL environnement variable");
    let jira_instance = jira::Jira::new(jira_url, jira_user, jira_password);
    jira_instance
        .add_work_load(args.issue, args.time, args.comment, args.started)
        .await;
    Ok(())
}

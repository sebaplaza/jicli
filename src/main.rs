use base64::encode;
use clap::Parser;
use reqwest::Client;
use std::env;
use std::{collections::HashMap, error::Error};

/// Simple JIRA client to add worklog time to an issue
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// key of jira issue
    #[clap(short, long)]
    issue: String,

    /// time (in the form: 1h, 1d, 30m, etc..)
    #[clap(short, long)]
    time: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let jira_user = env::var("JIRA_USER").expect("Please set $JIRA_USER environnement variable");
    let jira_password =
        env::var("JIRA_PASSWORD").expect("Please set $JIRA_PASSWORD environnement variable");
    let jira_url = env::var("JIRA_URL").expect("Please set $JIRA_URL environnement variable");

    let client = Client::new();

    let secret = encode([jira_user, jira_password].join(":"));
    // let secret = encode(format!("{}:{}", args.user, args.password));
    // let secret = encode(args.user + ":" + &args.password);

    let mut json_map = HashMap::new();
    json_map.insert("timeSpent", args.time);

    let url = format!("{}/rest/api/latest/issue/{}/worklog", jira_url, args.issue);
    let res = client
        .post(url)
        .json(&json_map)
        .header("Authorization", "Basic ".to_owned() + &secret)
        .send()
        .await?;
    // Parse the response body as Json in this case
    // let json = res.text().await?;
    // println!("{:?}", json);

    // println!("{} added to issue {}", args.time, args.issue);
    println!("time added to issue");
    Ok(())
}

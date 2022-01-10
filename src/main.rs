use base64::encode;
use clap::Parser;
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::error::Error;

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
}

struct Jira {
    url_base: String,
    url_api: String,
    user: String,
    password: String,
}

impl Jira {
    fn new(url: String, user: String, password: String) -> Jira {
        Jira {
            url_base: url.clone(),
            url_api: format!("{}/rest/api/latest", url),
            user,
            password,
        }
    }
    async fn call_api(&self, path: String, json_map: HashMap<&str, String>) {
        let url = format!("{}{}", self.url_api, path);
        let user_pass = format!("{}:{}", self.user, self.password);
        let secret = encode(user_pass);
        let client = Client::new();
        let authorization = "Basic ".to_owned() + &secret;
        client
            .post(url)
            .json(&json_map)
            .header("Authorization", authorization)
            .send()
            .await
            .expect("Problem calling the JIRA API");
    }
    async fn add_work_load(&self, issue: String, time: String) {
        let mut json_map = HashMap::new();
        json_map.insert("timeSpent", time.clone());
        let path = format!("/issue/{}/worklog", issue);
        self.call_api(path, json_map).await;
        let issue_url = format!("{}/browse/{}", self.url_base, issue);
        println!("'{}' has been added to issue {}", &time, &issue_url);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let jira_user = env::var("JIRA_USER").expect("Please set $JIRA_USER environnement variable");
    let jira_password =
        env::var("JIRA_PASSWORD").expect("Please set $JIRA_PASSWORD environnement variable");
    let jira_url = env::var("JIRA_URL").expect("Please set $JIRA_URL environnement variable");
    let jira = Jira::new(jira_url, jira_user, jira_password);
    jira.add_work_load(args.issue, args.time).await;
    Ok(())
}

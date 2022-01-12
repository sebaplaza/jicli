use base64::encode;
use reqwest::Client;
use std::collections::HashMap;

pub struct Jira {
    url_base: String,
    url_api: String,
    user: String,
    password: String,
}

impl Jira {
    pub fn new(url: String, user: String, password: String) -> Jira {
        Jira {
            url_base: url.clone(),
            url_api: format!("{}/rest/api/latest", url),
            user,
            password,
        }
    }
    async fn call_api(&self, path: String, json_map: HashMap<&str, String>) -> (u16, String) {
        let url = format!("{}{}", self.url_api, path);
        let user_pass = format!("{}:{}", self.user, self.password);
        let secret = encode(user_pass);
        let client = Client::new();
        let authorization = "Basic ".to_owned() + &secret;
        let res = client
            .post(url)
            .json(&json_map)
            .header("Authorization", authorization)
            .send()
            .await
            .expect("Problem calling the JIRA API");
        let status = res.status().as_u16();
        let data = res
            .text()
            .await
            .expect("Problem extracting data from API call");
        return (status, data);
    }
    pub async fn add_work_load(&self, issue: String, time: String, comment: String) {
        // Call Post api
        // https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-worklogs/#api-rest-api-3-issue-issueidorkey-worklog-post
        let mut json_map = HashMap::new();
        json_map.insert("timeSpent", time.clone());
        if !comment.is_empty() {
            json_map.insert("comment", comment.clone());
        }
        let path = format!("/issue/{}/worklog", issue);
        let (status, data) = self.call_api(path, json_map).await;
        if status >= 400 {
            eprintln!(
                "statusCode '{}' when calling API / details: {}",
                status, data
            );
            return;
        }
        let issue_url = format!("{}/browse/{}", self.url_base, issue);
        println!("'{}' has been added to issue {}", &time, &issue_url);
    }
}

# JICLI

Rust JIRA client

This is a simple JIRA client to helps to add worklog times into an issue.

## How to use it ?

Set 3 environnements variables

```sh
export JIRA_URL=https://my-jira.com
export JIRA_USER=my_user
export JIRA_PASSWORD=my_password
```

Then launch the client

```sh
jicli --issue JIRA-362 --time 1d
```
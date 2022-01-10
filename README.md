# JICLI

Rust JIRA client

This is a simple JIRA client to helps to add worklog times into an issue.


## Install it

```sh
cargo install jicli
```

## How to use it ?

Set these 3 environnements variables

```sh
export JIRA_URL=https://my-jira.com
export JIRA_USER=my_user
export JIRA_PASSWORD=my_password
```

Then launch the client

ex: add 1 day into issue JIRA-362
```sh
jicli --issue JIRA-362 --time 1d
```

# JICLI

Rust JIRA client

This is a simple JIRA client to helps to add worklog times into an issue.

## Requirements

You must a have rust tools ([rustup](https://rustup.rs/)) installed


## Install jicli via cargo

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

To know all possible options

```shell
jicli --help
```

## Integrations

For example, if your git brach includes the issue reference, you could write a shell function to deal with it automatically.

If your git branch looks like `feat/JIRA-234-my-awesome-feature` , you can write a bash function like this

Add this to your `~/.bashrc` or `~/.zshrc`

```bash
# jira time function
# ex: jt 1h   (to add one hour to ticket)
function jt() {
  # extract jira ticket from branch and add worklog time
  jicli --time $1 --issue $(git branch --show-current --no-color | grep  -Po '[A-Z]+-[0-9]+')
}
```

Then, use it like this:

```sh
# add 1 day to issue JIRA-234
jt 1d
```

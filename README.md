# Mjölnir

[![Build Status](https://travis-ci.org/ChrisMacNaughton/Mjolnir.svg?branch=master)](https://travis-ci.org/ChrisMacNaughton/Mjolnir)<!--
[![Build status](https://ci.appveyor.com/api/projects/status/5wcdaupe9dva9tcf?svg=true)](https://ci.appveyor.com/project/ChrisMacNaughton/mjolnir)
-->
[![Version info](https://img.shields.io/crates/v/mjolnir.svg)](https://crates.io/crates/mjolnir)
[![Coverage Status](https://coveralls.io/repos/github/ChrisMacNaughton/Mjolnir/badge.svg?branch=master)](https://coveralls.io/github/ChrisMacNaughton/Mjolnir?branch=master)

Mjölnir is a tool to solve problems on large numbers of servers.

See [the design document](DESIGN.md) to learn about the architecture and design of Mjölnir.

Mjolnir is designed to be agnostic to issues and the solutions to them; however, it helps server administrators to map incoming alerts into one or more remediations that can be attempted. If a requested remediation fails for any reason, Mjolnir will attempt to run another remediation from the supplied pipeline until there are no more to try. It is suggested to add a plugin that alerts your humans in the final pipeline slot to ensure that the problems are dealt with.

After a new problem has been identified, it should be fairly trivial to create a new plugin to solve the problem, and add it to the configured pipelines, allowing the administrators to solve a problem once, and then let Mjolnir solve it any time it happens in the future!

## Getting Started

A useful starting point for setting up Mjolnir may be:

```toml
[mjolnir]
  # key_path is where Mjolnir will store the secret keys used to encrypt
  # and authenticate communication after the initial handshake has completed
  key_path = "/usr/local/share/mjolnir"
  # Masters should reference all master nodes configured to run mjolnird
  # in master mode
  masters = ["$MASTER_IP:11011:12011"]
  # plugin_path is the path on the master node(s) that plugins arelocated at
  plugin_path = "/build/target/debug/examples"
  # secret is a shared secret that is used to authenticate communication.
  #
  # YOU SHOULD CHANGE THIS
  #
  secret = "w[4957ha[ruognqp357gf;eruigap47gfa;IRYEgf0a864fo"

# default_remediation allows Mjolnir to fall back to this remediation if
# no other configured remediation has resolved an alert. This should be
# configured to match any existing notification tools (eg: slack, email)
[mjolnir.default_remediation]
  plugin = "slack"
  args = ["botname=user", "channel=#general", "webhook=https://PATH"]

[master]
bind = "0.0.0.0:11011:12011"

[agent]
bind = "0.0.0.0:11012:12012"

[[pipelines]]

  # pipelines.actions is an array of actions that will be attempted,
  # in order, to try to remediate this alert.
  [[pipelines.actions]]
    plugin = "clean_disk"
    # args is an optional configuration option for a remediation that can be
    # taken. They will be passed into the plugin on call so you should check
    # the documentation of the configured plugin to see what options it expects
    args = ["path=/var/log"]

  # pipelines.trigger is the event that will trigger the above configured
  # actions.
  [pipelines.trigger]
    type = "alertmanager"
    name = "full-disk"
```

## Specifying the plugin pipeline

Pipelines are specified in the controllers configuration file using toml syntax.  Triggers are matched up against
actions.  Multiple actions can be specified and they will be tried in order until one succeeds.

```toml
[[pipelines]]
  [[pipelines.actions]]
    plugin = "clean_disk"
  [pipelines.trigger]
    type = "alertmanager"
    name = "full-disk" # This corresponds to the controllers webhook.  {controller_ip}:{controller_port}/webhook/full-disk
[[pipelines]]
  [[pipelines.actions]]
    plugin = "reboot_server"
  [pipelines.trigger]
    type = "alertmanager"
    name = "server-down"
```
## Plugins


### Alerting

Coming Soon!

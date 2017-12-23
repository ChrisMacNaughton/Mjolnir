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

## Plugins

### Alerting

- [Email](https://gogs.centauri.solutions/Mjolnir/email)
- [Grafana](https://gogs.centauri.solutions/Mjolnir/grafana)
- [Slack](https://gogs.centauri.solutions/Mjolnir/slack)
- [Telegram](https://gogs.centauri.solutions/Mjolnir/telegram)

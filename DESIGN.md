# Mjölnir

Mjölnir is divided up into a few pieces, [controllers](#controller), [agents](#agents), and [plugins](plugins). These parts are explained in more detail below.

## Controller

The controller connects all of the different pieces together, and hosts the [plugins](plugins) so that the [agents](#agents) can download them as needed. In addition to connecting things, the controller(s) control what actions are taken from the alerting inputs.

## Agents

Agents are lightweight binaries that can be run on all servers in a datacenter, and can download various remediation plugins from the controller(s) to take the required actions.

## Plugins

Plugins are how Mjölnir interacts with other things, from monitoring frameworks like Prometheus to datacenter management software like MAAS, or even cloud APIs like AWS.

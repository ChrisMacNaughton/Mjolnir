# Mjölnir

Mjölnir is divided up into a few pieces, [controllers](#controller), [agents](#agents), and [plpugins](plugins). These parts are explained in more detail below.

## Controller

The controller connects all of the different pieces together, and hosts the [plpugins](plugins) so that the [agents](#agents) can download them as needed. In addition to connecting things, the controller(s) control what aactions are taken from the alerting inputs.

## Agents

Agents are lightweight binaries that can be run on all servers in a datacenter, and can download various remediation plugins from the controller(s) to take the required actions.  Agents listen on a tcp port for a protocol buffer request before taking action.  Afterwards they respond with a protocol buffer response detailing if their actions were successful or not.  The controller can then decide to escalate if needed.  

## Plugins

Plugins are how Mjölnir interacts with other things, from monitoring frameworks like Prometheus to datacenter management software like MAAS, or even cloud APIs like AWS.

## Communication

All communication is specified in the [api] document. 

[api]: https://github.com/ChrisMacNaughton/Mjolnir/blob/master/api/protos/api.proto

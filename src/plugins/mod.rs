use mjolnir_api::{self, plugin};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginEntry {
    pub name: String,
    pub author: String,
    pub version: String,
    pub webhook: bool,
    pub alerts: Vec<Alert>,
    pub remediations: Vec<Remediation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alert {
    pub title: String,
    pub name: Option<String>,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remediation {
    plugin: String,
    target: Option<String>,
    args: Vec<String>,
}

impl PluginEntry {
    pub fn try_from(input: &[u8]) -> Option<PluginEntry> {
        match plugin::Discover::try_from(input) {
            Ok(entry) => Some(entry.into()),
            Err(e) => {println!("Problem parsing: {:?}", e); None}
        }
    }
}
impl From<plugin::Discover> for PluginEntry {
    fn from(plugin: plugin::Discover) -> PluginEntry {
        let repeated_alerts = plugin.get_alerts();
        // let mut alerts = Vec::with_capacity(repeated_alerts.len());
        let alerts = repeated_alerts.iter().map(|alert| alert.into() ).collect();

        let repeated_actions = plugin.get_actions();
        // let mut actions = Vec::with_capacity(repeated_alerts.len());
        let actions = repeated_actions.iter().map(|action| action.into() ).collect();

        PluginEntry {
            name: plugin.get_name().into(),
            author: plugin.get_author().into(),
            version: plugin.get_version().into(),
            webhook: plugin.get_webhook(),
            alerts: alerts,
            remediations: actions,
        }
    }
}

impl From<PluginEntry> for plugin::Discover {
    fn from(plugin: PluginEntry) -> plugin::Discover {
        let mut d = plugin::Discover::default();
        d.set_name(plugin.name);
        d.set_author(plugin.author);
        d.set_version(plugin.version);
        
        let mut repeated_alerts = mjolnir_api::RepeatedField::default();
        for alert in plugin.alerts {
            repeated_alerts.push(alert.into());
        }
        d.set_alerts(repeated_alerts);
        d
    }
}

impl<'a> From<&'a plugin::Alert> for Alert {
    fn from(alert: &plugin::Alert) -> Alert {
        Alert {
            title: alert.get_title().into(),
            name: if alert.has_dynamic_name() {
                    Some(alert.get_dynamic_name().to_string())
                } else {
                    None
                },
            source: if alert.has_source() {
                    Some(alert.get_source().to_string())
                } else {
                    None
                },
        }
    }
}

impl From<Alert> for plugin::Alert {
    fn from(alert: Alert) -> plugin::Alert {
        let mut a = plugin::Alert::default();
        a.set_title(alert.title);
        if let Some(name) = alert.name {
            a.set_dynamic_name(name);
        }
        if let Some(source) = alert.source {
            a.set_source(source);
        }
        
        // d.set_alerts()
        a
    }
}

impl<'a> From<&'a plugin::RemediationRequest> for Remediation {
    fn from(remediation: &plugin::RemediationRequest) -> Remediation {
        Remediation {
            plugin: remediation.get_plugin().into(),
            target: if remediation.has_target() {
                    Some(remediation.get_target().to_string())
                } else {
                    None
                },
            args: remediation.get_args().into()
        }
    }
}


impl From<Remediation> for plugin::RemediationRequest {
    fn from(remediation: Remediation) -> plugin::RemediationRequest {
        let mut a = plugin::RemediationRequest::default();
        a.set_plugin(remediation.plugin);
        if let Some(target) = remediation.target {
            a.set_target(target);
        }
        let mut repeated_args = mjolnir_api::RepeatedField::default();
        for arg in remediation.args {
            repeated_args.push(arg.into());
        }
        a.set_args(repeated_args);
        a
    }
}
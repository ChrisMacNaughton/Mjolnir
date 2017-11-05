use std::path::PathBuf;
use plugin;
use RepeatedField;
use Alert;
use Remediation;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_a_plugin_entry_from_protobuf() {
        let buff = [
            10,
            12,
            97,
            108,
            101,
            114,
            116,
            109,
            97,
            110,
            97,
            103,
            101,
            114,
            18,
            46,
            67,
            104,
            114,
            105,
            115,
            32,
            77,
            97,
            99,
            78,
            97,
            117,
            103,
            104,
            116,
            111,
            110,
            32,
            60,
            99,
            104,
            114,
            105,
            115,
            64,
            99,
            101,
            110,
            116,
            97,
            117,
            114,
            105,
            115,
            111,
            108,
            117,
            116,
            105,
            111,
            110,
            115,
            46,
            110,
            108,
            62,
            26,
            5,
            48,
            46,
            48,
            46,
            49,
            40,
            1,
        ];
        let plugin = PluginEntry::try_from(&buff, PathBuf::from("/tmp")).unwrap();
        println!("Plugin: {:?}", plugin);
        assert_eq!(plugin.name, "alertmanager");
        assert!(plugin.webhook);
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
    pub path: PathBuf,
}

impl PluginEntry {
    pub fn try_from(input: &[u8], path: PathBuf) -> Option<PluginEntry> {
        match plugin::Discover::try_from(input) {
            Ok(entry) => {
                let p: PluginEntry = entry.into();
                Some(p.with_path(path))
            }
            Err(e) => {
                println!("Problem parsing: {:?}", e);
                None
            }
        }
    }

    fn with_path(mut self, path: PathBuf) -> PluginEntry {
        self.path = path;
        self
    }
}

impl From<plugin::Discover> for PluginEntry {
    fn from(plugin: plugin::Discover) -> PluginEntry {
        let repeated_alerts = plugin.get_alerts();
        let alerts = repeated_alerts.iter().map(|alert| alert.into()).collect();

        let repeated_actions = plugin.get_actions();
        let actions = repeated_actions
            .iter()
            .map(|action| action.into())
            .collect();

        PluginEntry {
            name: plugin.get_name().into(),
            author: plugin.get_author().into(),
            version: plugin.get_version().into(),
            webhook: plugin.get_webhook(),
            alerts: alerts,
            remediations: actions,
            path: PathBuf::from(""),
        }
    }
}

impl From<PluginEntry> for plugin::Discover {
    fn from(plugin: PluginEntry) -> plugin::Discover {
        let mut d = plugin::Discover::default();
        d.set_name(plugin.name);
        d.set_author(plugin.author);
        d.set_version(plugin.version);

        let mut repeated_alerts = RepeatedField::default();
        for alert in plugin.alerts {
            repeated_alerts.push(alert.into());
        }
        d.set_alerts(repeated_alerts);
        d
    }
}

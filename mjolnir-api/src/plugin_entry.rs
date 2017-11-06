use std::path::PathBuf;
use plugin;
use RepeatedField;
use Alert;
use Remediation;

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes() {
        let plugin = PluginEntry {
            name: "test-name".into(),
            author: "test-author".into(),
            version: "test-version".into(),
            webhook: false,
            alerts: vec![
                Alert {
                    title: "Test1".into(),
                    name: None,
                    source: Some("test".into()),
                },Alert {
                    title: "Test2".into(),
                    name: None,
                    source: Some("test".into()),
                }
            ],
            remediations: vec![
                Remediation {
                    plugin: "Test".into(),
                    target: Some("awesomehost.local".into()),
                    args: vec!["body".into()],
                },
            ],
            path: PathBuf::from("/tmp/test"),
        };

        let request: plugin::Discover = plugin.clone().into();

        let bytes = request.write_to_bytes().expect("Couldn't turn the plugin into bytes");
        let mut plugin2: PluginEntry = parse_from_bytes::<plugin::Discover>(&bytes).unwrap().into();
        plugin2 = plugin2.with_path(PathBuf::from("/tmp/test"));
        assert_eq!(plugin, plugin2);
    }

    #[test]
    fn it_fails_parsing_gracefully() {
        let buff = [12];
        let plugin = PluginEntry::try_from(&buff, PathBuf::from("/tmp"));
        assert!(plugin.is_err());
    }

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
    pub fn try_from(input: &[u8], path: PathBuf) -> Result<PluginEntry,String> {
        match plugin::Discover::try_from(input) {
            Ok(entry) => {
                let p: PluginEntry = entry.into();
                Ok(p.with_path(path))
            }
            Err(e) => {
                Err(format!("Problem parsing: {:?}", e))
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
        d.set_webhook(plugin.webhook);

        let mut repeated_alerts = RepeatedField::default();
        for alert in plugin.alerts {
            repeated_alerts.push(alert.into());
        }
        d.set_alerts(repeated_alerts);

        let mut repeated_remediations = RepeatedField::default();
        for alert in plugin.remediations {
            repeated_remediations.push(alert.into());
        }
        d.set_actions(repeated_remediations);

        d
    }
}

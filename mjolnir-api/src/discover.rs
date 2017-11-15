use proto;
use protobuf::{self, Message};

use {Alert, Remediation};

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes() {
        let discover = Discover::new("test");

        let request: proto::plugin::Discover = discover.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let discover2 = parse_from_bytes::<proto::plugin::Discover>(&bytes).unwrap().into();
        assert_eq!(discover, discover2);
    }

    #[test]
    fn it_serializes_and_deserializes_with_optionals() {
        let discover = Discover::new("test")
            .with_author("Test")
            .with_version("0.0.1")
            .with_alert(Alert {
                alert_type: "Test".into(),
                name: Some("placeholder".into()),
                source: Some("test".into()),
                args: vec!["testarg=value".into()],
                next_remediation: 0,
            })
            .with_remediation(Remediation {
                plugin: "Test".into(),
                target: None,
                args: vec!["body".into()],
                alert: None,
            });

        let request: proto::plugin::Discover = discover.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let discover2 = parse_from_bytes::<proto::plugin::Discover>(&bytes).unwrap().into();
        assert_eq!(discover, discover2);
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Discover {
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub alerts: Vec<Alert>,
    pub webhook: bool,
    pub remediations: Vec<Remediation>,
}


impl Discover {
    pub fn new<T: Into<String>>(name: T) -> Discover {
       let mut d = Discover::default();
       d.name = name.into();
       d
    }

    pub fn with_author<T: Into<String>>(mut self, author: T) -> Self {
        self.author = Some(author.into());
        self
    }
    
    pub fn with_version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_alert(mut self, alert: Alert) -> Self {
        self.alerts.push(alert);
        self
    }

    pub fn with_alerts(mut self, alerts: Vec<Alert>) -> Self {
        self.alerts = alerts;
        self
    }

    pub fn webhook(mut self) -> Self {
        self.webhook = true;
        self
    }

    pub fn with_remediation(mut self, remediation: Remediation) -> Self {
        self.remediations.push(remediation);
        self
    }

    pub fn with_remediations(mut self, remediations: Vec<Remediation>) -> Self {
        self.remediations = remediations;
        self
    }

    pub fn write_to_bytes(self) -> Result<Vec<u8>, protobuf::ProtobufError> {
        let plugin_result: proto::plugin::Discover = self.into();

        plugin_result.write_to_bytes()
    }
}

impl Into<proto::plugin::Discover> for Discover {
    fn into(self) -> proto::plugin::Discover {
        let mut discover = proto::plugin::Discover::new();
        discover.set_name(self.name);
        if let Some(author) = self.author {
            discover.set_author(author);
        }
        if let Some(version) = self.version {
            discover.set_version(version);
        }
        discover.set_alerts(Alert::vec_to_repeated(&self.alerts));
        discover.set_webhook(self.webhook);
        discover.set_actions(Remediation::vec_to_repeated(&self.remediations));
        discover
    }
}

impl From<proto::plugin::Discover> for Discover {
    fn from(discover: proto::plugin::Discover) ->Discover {
        Discover {
            name: discover.get_name().to_owned(),
            author: if discover.has_author() {
                Some(discover.get_author().to_owned())
            } else {
                None
            },
            version: if discover.has_version() {
                Some(discover.get_version().to_owned())
            } else {
                None
            },
            alerts: discover.get_alerts().iter().map(|a| a.into()).collect(),
            webhook: discover.get_webhook(),
            remediations: discover.get_actions().iter().map(|a| a.into()).collect(),
        }
    }
}
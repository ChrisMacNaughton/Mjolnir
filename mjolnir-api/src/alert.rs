use super::proto::plugin;

use uuid::Uuid;

use protobuf;

use crate::{Message, RepeatedField, parse_from_bytes};

#[cfg(test)]
mod tests {
    use super::*;

    use toml;

    use crate::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes() {
        let alert = Alert {
            alert_type: "Test".into(),
            name: Some("placeholder".into()),
            source: Some("test".into()),
            args: vec!["testarg=value".into()],
            next_remediation: 0,
            uuid: uuid(),
        };

        let request: plugin::Alert = alert.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let alert2 = parse_from_bytes::<plugin::Alert>(&bytes).unwrap().into();
        assert_eq!(alert, alert2);
    }

    #[test]
    fn it_constructs() {
        let mut alert = Alert::default();

        alert = alert.with_arg("test1=2".into());
        assert_eq!(alert.args, vec!["test1=2"]);

        let args = vec!["test=1".into(), "test2=2".into()];
        alert = alert.with_args(args.clone());
        assert_eq!(alert.args, vec!["test1=2", "test=1", "test2=2"]);

        assert_eq!(alert.next_remediation, 0);
        alert = alert.increment();
        assert_eq!(alert.next_remediation, 1);

        let alert2 = Alert::from_string(&String::from_utf8_lossy(
            &alert.clone().write_to_bytes().unwrap(),
        ).into_owned());
        assert_eq!(alert, alert2);
    }

    #[test]
    fn it_serializes_and_deserializes_without_optionals() {
        let alert = Alert {
            alert_type: "Test".into(),
            name: None,
            source: None,
            args: vec![],
            next_remediation: 0,
            uuid: uuid(),
        };

        let request: plugin::Alert = alert.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let alert2 = parse_from_bytes::<plugin::Alert>(&bytes).unwrap().into();
        assert_eq!(alert, alert2);
    }

    #[test]
    fn it_can_convert_from_vec() {
        let r = vec![
            Alert {
                alert_type: "Test".into(),
                name: None,
                source: None,
                args: vec![],
                next_remediation: 0,
                uuid: uuid(),
            },
        ];

        let repeated = Alert::vec_to_repeated(&r);
        assert_eq!(r[0], repeated.first().unwrap().into());
    }

    #[test]
    fn it_deserializes_alert_from_toml() {
        let s = r#"
type = "alertmanager"
name = "full-disk""#;

        let alert: Alert = toml::from_str(s).unwrap();
        println!("Alert: {:?}", alert);
    }


    #[test]
    fn empty_vec() {
        let empty_vec: Vec<String> = vec![];
        assert_eq!(empty_vec, empty());
    }

    #[test]
    fn is_zero() {
        assert_eq!(0, zero());
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq)]
pub struct Alert {
    /// In config, this is referred to as type
    #[serde(rename = "type")]
    pub alert_type: String,
    pub name: Option<String>,
    pub source: Option<String>,
    #[serde(default = "empty")]
    pub args: Vec<String>,
    /// Master managed index into pipeline
    #[serde(default = "zero")]
    pub next_remediation: u64,
    #[serde(default = "uuid")]
    pub uuid: Uuid,
}

fn uuid() -> Uuid {
    Uuid::new_v4()
}

fn empty() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn zero() -> u64 {
    0
}

impl Default for Alert {
    fn default() -> Alert {
        Alert {
            alert_type: String::new(),
            name: None,
            source: None,
            args: vec![],
            next_remediation: 0,
            uuid: Uuid::new_v4(),
        }
    }
}
impl PartialEq for Alert {
    fn eq(&self, other: &Alert) -> bool {
        self.alert_type == other.alert_type && self.name == other.name
    }
}


impl<'a> From<&'a plugin::Alert> for Alert {
    fn from(alert: &plugin::Alert) -> Alert {
        Alert {
            alert_type: alert.get_alert_type().into(),
            name: if alert.has_name() {
                Some(alert.get_name().to_string())
            } else {
                None
            },
            source: if alert.has_source() {
                Some(alert.get_source().to_string())
            } else {
                None
            },
            args: alert.get_args().into(),
            next_remediation: alert.get_next_remediation(),
            uuid: alert.get_uuid().into(),
        }
    }
}

impl From<plugin::Alert> for Alert {
    fn from(alert: plugin::Alert) -> Alert {
        (&alert).into()
    }
}

impl<'a> From<&'a Alert> for plugin::Alert {
    fn from(alert: &Alert) -> plugin::Alert {
        let mut a = plugin::Alert::default();
        a.set_alert_type(alert.alert_type.clone());
        if let Some(ref name) = alert.name.clone() {
            a.set_name(name.clone());
        }
        if let Some(ref source) = alert.source {
            a.set_source(source.clone());
        }
        let mut repeated_args = RepeatedField::default();
        for arg in alert.args.clone() {
            repeated_args.push(arg.into());
        }
        a.set_args(repeated_args);
        a.set_next_remediation(alert.next_remediation);
        a.set_uuid(alert.uuid.into());
        // d.set_alerts()
        a
    }
}

impl From<Alert> for plugin::Alert {
    fn from(alert: Alert) -> plugin::Alert {
        (&alert).into()
    }
}

// impl From<Vec<Alert>> for RepeatedField<plugin::Alert> {
//     fn from(alerts: Vec<Alert>) -> RepeatedField<plugin::Alert> {
//         let mut repeated_alerts = RepeatedField::default();
//         for alert in alerts {
//             repeated_alerts.push(alert.into());
//         }
//         repeated_alerts
//     }
// }

impl Alert {
    pub fn vec_to_repeated(alerts: &Vec<Alert>) -> RepeatedField<plugin::Alert> {
        let mut repeated_alerts = RepeatedField::default();
        for alert in alerts {
            repeated_alerts.push(alert.into());
        }
        repeated_alerts
    }

    pub fn new<T: Into<String>>(alert_type: T) -> Alert {
        Alert::default().with_alert_type(alert_type)
    }

    pub fn with_alert_type<T: Into<String>>(mut self, alert_type: T) -> Self {
        self.alert_type = alert_type.into();
        self
    }

    pub fn with_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_source<T: Into<String>>(mut self, source: T) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_args(mut self, mut args: Vec<String>) -> Self {
        self.args.append(&mut args);
        self
    }

    pub fn with_arg(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }

    pub fn increment(mut self) -> Self {
        self.next_remediation += 1;
        self
    }

    pub fn write_to_bytes(self) -> Result<Vec<u8>, protobuf::ProtobufError> {
        let plugin_result: plugin::Alert = self.into();

        plugin_result.write_to_bytes()
    }

    pub fn from_string(input: &String) -> Alert {
        let r2 = parse_from_bytes::<plugin::Alert>(input.as_bytes()).unwrap();
        r2.into()
    }
}

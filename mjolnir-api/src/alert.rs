use super::proto::plugin;

use RepeatedField;

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes() {
        let alert = Alert {
            alert_type: "Test".into(),
            name: Some("placeholder".into()),
            source: Some("test".into()),
        };

        let request: plugin::Alert = alert.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let alert2 = parse_from_bytes::<plugin::Alert>(&bytes).unwrap().into();
        assert_eq!(alert, alert2);
    }

    #[test]
    fn it_serializes_and_deserializes_without_optionals() {
        let alert = Alert {
            alert_type: "Test".into(),
            name: None,
            source: None,
        };

        let request: plugin::Alert = alert.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let alert2 = parse_from_bytes::<plugin::Alert>(&bytes).unwrap().into();
        assert_eq!(alert, alert2);
    }

    #[test]
    fn it_can_convert_from_vec() {
        let r = vec![Alert {
            alert_type: "Test".into(),
            name: None,
            source: None,
        }];

        let repeated = Alert::vec_to_repeated(&r);
        assert_eq!(r[0], repeated.first().unwrap().into());
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Alert {
    /// In config, this is referred to as type
    pub alert_type: String,
    pub name: Option<String>,
    pub source: Option<String>,
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
}
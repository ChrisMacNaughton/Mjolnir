use super::proto::plugin;

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes() {
        let alert = Alert {
            title: "Test".into(),
            name: Some("placeholder".into()),
            source: Some("test".into()),
        };

        let request: plugin::Alert = alert.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let alert2 = parse_from_bytes::<plugin::Alert>(&bytes).unwrap().into();
        assert_eq!(alert, alert2);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alert {
    pub title: String,
    pub name: Option<String>,
    pub source: Option<String>,
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

impl From<plugin::Alert> for Alert {
    fn from(alert: plugin::Alert) -> Alert {
        (&alert).into()
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

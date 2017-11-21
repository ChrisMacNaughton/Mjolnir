use protobuf;

use plugin::{self, RemediationRequest};
use alert::Alert;
use {Message, RepeatedField, parse_from_bytes};

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes() {
        let remediation = Remediation {
            plugin: "Test".into(),
            target: Some("awesomehost.local".into()),
            args: vec!["body".into()],
            alert: Some(Alert {
                    alert_type: "Test1".into(),
                    name: None,
                    source: Some("test".into()),
                    args: vec![],
                    next_remediation: 0,
            }),
        };

        let request: RemediationRequest = remediation.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let r2 = parse_from_bytes::<RemediationRequest>(&bytes).unwrap();
        let remediation2 = r2.into();
        assert_eq!(remediation, remediation2);
    }

    #[test]
    fn it_serializes_and_deserializes_without_optionals() {
        let remediation = Remediation {
            plugin: "Test".into(),
            target: None,
            args: vec!["body".into()],
            alert: None,
        };

        let request: RemediationRequest = remediation.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let r2 = parse_from_bytes::<RemediationRequest>(&bytes).unwrap();
        let remediation2 = r2.into();
        assert_eq!(remediation, remediation2);
    }

    #[test]
    fn it_can_convert_from_vec() {
        let r = vec![Remediation {
            plugin: "Test".into(),
            target: None,
            args: vec!["body".into()],
            alert: None,
        }];

        let repeated = Remediation::vec_to_repeated(&r);
        assert_eq!(r[0], repeated.first().unwrap().into());
    }

    #[test]
    fn empty_vec() {
        let empty_vec: Vec<String> = vec![];
        assert_eq!(empty_vec, empty());
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Remediation {
    pub plugin: String,
    pub target: Option<String>,
    #[serde(default="empty")]
    pub args: Vec<String>,
    pub alert: Option<Alert>,
}

fn empty() -> Vec<String> {
    vec![]
}


impl<'a> From<&'a RemediationRequest> for Remediation {
    fn from(remediation: &RemediationRequest) -> Remediation {
        let alert = if remediation.has_alert() {
            Some(remediation.get_alert().into())
        } else {
            None
        };
        Remediation {
            plugin: remediation.get_plugin().into(),
            target: if remediation.has_target() {
                Some(remediation.get_target().to_string())
            } else {
                None
            },
            args: remediation.get_args().into(),
            alert: alert,
        }
    }
}

impl From<RemediationRequest> for Remediation {
    fn from(remediation: RemediationRequest) -> Remediation {
        (&remediation).into()
    }
}


impl From<Remediation> for RemediationRequest {
    fn from(remediation: Remediation) -> RemediationRequest {
        (&remediation).into()
    }
}

impl<'a> From<&'a Remediation> for RemediationRequest {
    fn from(remediation: &Remediation) -> RemediationRequest {
        let mut a = RemediationRequest::default();
        a.set_plugin(remediation.plugin.clone());
        if let Some(ref target) = remediation.target {
            a.set_target(target.clone());
        }
        let mut repeated_args = RepeatedField::default();
        for arg in remediation.args.clone() {
            repeated_args.push(arg.into());
        }
        a.set_args(repeated_args);
        if let Some(ref alert) = remediation.alert {
            a.set_alert(alert.clone().into());
        }
        a
    }
}

impl Remediation {
    pub fn vec_to_repeated(remediations: &Vec<Remediation>) -> RepeatedField<RemediationRequest> {
        let mut repeated_remediations = RepeatedField::default();
        for remediation in remediations {
            repeated_remediations.push(remediation.into());
        }
        repeated_remediations
    }

    pub fn write_to_bytes(self) -> Result<Vec<u8>, protobuf::ProtobufError> {
        let plugin_result: plugin::RemediationRequest = self.into();

        plugin_result.write_to_bytes()
    }

    pub fn from_string(input: &String) -> Remediation {
        let r2 = parse_from_bytes::<plugin::RemediationRequest>(input.as_bytes()).unwrap();
        r2.into()
    }

    pub fn from_bytes(input: &[u8]) -> Remediation {
        let r2 = parse_from_bytes::<plugin::RemediationRequest>(input).unwrap();
        r2.into()
    }
}
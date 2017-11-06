use plugin::RemediationRequest;
use RepeatedField;

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
        };

        let request: RemediationRequest = remediation.clone().into();

        let bytes = request.write_to_bytes().unwrap();
        let r2 = parse_from_bytes::<RemediationRequest>(&bytes).unwrap();
        let remediation2 = r2.into();
        assert_eq!(remediation, remediation2);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remediation {
    pub plugin: String,
    pub target: Option<String>,
    pub args: Vec<String>,
}

impl<'a> From<&'a RemediationRequest> for Remediation {
    fn from(remediation: &RemediationRequest) -> Remediation {
        Remediation {
            plugin: remediation.get_plugin().into(),
            target: if remediation.has_target() {
                Some(remediation.get_target().to_string())
            } else {
                None
            },
            args: remediation.get_args().into(),
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
        let mut a = RemediationRequest::default();
        a.set_plugin(remediation.plugin);
        if let Some(target) = remediation.target {
            a.set_target(target);
        }
        let mut repeated_args = RepeatedField::default();
        for arg in remediation.args {
            repeated_args.push(arg.into());
        }
        a.set_args(repeated_args);
        a
    }
}

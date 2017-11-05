use plugin::RemediationRequest;
use RepeatedField;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remediation {
    plugin: String,
    target: Option<String>,
    args: Vec<String>,
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

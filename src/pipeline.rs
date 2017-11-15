use mjolnir_api::{Alert, Remediation};

#[derive(Clone, Debug, PartialEq)]
pub struct Pipeline {
    pub trigger: Alert,
    pub actions: Vec<Remediation>,
}
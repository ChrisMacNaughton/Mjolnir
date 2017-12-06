use mjolnir_api::{Alert, Remediation};

use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn it_parses_a_pipeline_from_toml() {
        let s = r#"
[trigger]
type = "alertmanager" # The type of alert to match on
name = "full-disk" # Optionally, the name that the alert provides

[[actions]]

plugin = "clean-disk"

[[actions]]

plugin = "alert""#;
        let pipeline: Pipeline = toml::from_str(s).unwrap();
        println!("Pipeline: {:?}", pipeline);
    }

    #[test]
    fn empty_vec() {
        let empty_vec: Vec<Remediation> = vec![];
        assert_eq!(empty_vec, empty());
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Pipeline {
    pub trigger: Alert,
    #[serde(default="empty")]
    pub actions: Vec<Remediation>,
    #[serde(default="uuid")]
    uuid: Uuid,
}

fn empty() -> Vec<Remediation> {
    vec![]
}

fn uuid() -> Uuid {
    Uuid::new_v4()
}
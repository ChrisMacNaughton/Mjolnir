use mjolnir_api::{Alert, Remediation};

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
    fn it_parses_a_vec_pipeline_from_toml() {
        let s = r#"
[[pipelines]]

  [[pipelines.actions]]
    plugin = "clean_disk"

  [[pipelines.actions]]
    plugin = "alert"

  [pipelines.trigger]
    type = "alertmanager"
    name = "full-disk""#;
        let pipeline: Vec<Pipeline> = toml::from_str(s).unwrap();
        println!("Pipeline: {:?}", pipeline);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Pipeline {
    pub trigger: Alert,
    #[serde(default="empty")]
    pub actions: Vec<Remediation>,
}

fn empty() -> Vec<Remediation> {
    vec![]
}
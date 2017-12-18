use mjolnir_api::{Alert, Remediation};

use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn it_parses_alert_pipelines() {
        let s = r#"
[trigger]
type = "alertmanager" # The type of alert to match on
name = "full-disk" # Optionally, the name that the alert provides

[[actions]]

plugin = "clean_disk"

[[actions]]

plugin = "alert""#;
        let pipeline: Pipeline = toml::from_str(s).unwrap();
        println!("Pipeline: {:?}", pipeline);

        assert_eq!(pipeline.trigger, Trigger::Alert(Alert::new("alertmanager").with_name("full-disk")));
        assert_eq!(pipeline.actions.iter().map(|ref a| &a.plugin).collect::<Vec<&String>>(), vec!["clean_disk", "alert"]);
    }

    #[test]
    fn it_parses_timer_pipelines() {
        let s = r#"
  [trigger]
    timer = "0 5 * * *" # 5 AM daily

  [[actions]]
    plugin = "backup"
    args = ["path=/home/chris", "proto=zfs"]
"#;
        let pipeline: Pipeline = toml::from_str(s).unwrap();
        println!("Pipeline: {:?}", pipeline);
        assert_eq!(pipeline.trigger, Trigger::Timer{timer: "0 5 * * *".into()});
        assert_eq!(pipeline.actions.iter().map(|ref a| &a.plugin).collect::<Vec<&String>>(), vec!["backup"]);
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Test {
        pipelines: Vec<Pipeline>,
    }

    #[test]
    fn it_parses_complex_pipelines() {
        let s = r#"
[[pipelines]]

  [pipelines.trigger]
    type = "alertmanager"
    name = "full-disk"

  [[pipelines.actions]]
    plugin = "clean_disk"

  [[pipelines.actions]]
    plugin = "alert"
    
[[pipelines]]
  
  [pipelines.trigger]
    timer = "0 5 * * *" # 5 AM daily

  [[pipelines.actions]]
    plugin = "backup"
    args = ["path=/home/chris", "proto=zfs"]
"#;
        let test: Test = toml::from_str(s).unwrap();
        let pipelines = test.pipelines;
        println!("Pipeline: {:?}", pipelines);

        let pipeline = &pipelines[0];
        assert_eq!(pipeline.trigger, Trigger::Alert(Alert::new("alertmanager").with_name("full-disk")));
        assert_eq!(pipeline.actions.iter().map(|ref a| &a.plugin).collect::<Vec<&String>>(), vec!["clean_disk", "alert"]);

        let pipeline = &pipelines[1];
        assert_eq!(pipeline.trigger, Trigger::Timer{timer: "0 5 * * *".into()});
        assert_eq!(pipeline.actions.iter().map(|ref a| &a.plugin).collect::<Vec<&String>>(), vec!["backup"]);
    }

    #[test]
    fn empty_vec() {
        let empty_vec: Vec<Remediation> = vec![];
        assert_eq!(empty_vec, empty());
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Trigger {
    Alert(Alert),
    Timer { timer: String },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Pipeline {
    pub trigger: Trigger,
    #[serde(default = "empty")]
    pub actions: Vec<Remediation>,
    #[serde(default = "uuid")]
    uuid: Uuid,
}

fn empty() -> Vec<Remediation> {
    vec![]
}

fn uuid() -> Uuid {
    Uuid::new_v4()
}

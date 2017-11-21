use protobuf;

use {Message, RepeatedField, parse_from_bytes};

#[cfg(test)]
mod tests {
    use super::*;

    pub use protobuf::core::{Message, parse_from_bytes};

    #[test]
    fn it_serializes_and_deserializes_ok() {
        let result = RemediationResult {
            result: Ok(()),
            alerts: vec![],
        };

        let plugin_result: plugin::RemediationResult = result.clone().into();

        let bytes = plugin_result.write_to_bytes().unwrap();
        let r2 = parse_from_bytes::<plugin::RemediationResult>(&bytes).unwrap();
        let result2 = r2.into();
        assert_eq!(result, result2);
    }


    #[test]
    fn it_builds() {
        let mut r = RemediationResult::new();
        r = r.ok();
        assert!(r.result.is_ok());
        r = r.err("Error!");
        assert!(r.result.is_err());
        assert_eq!(r.result, Err("Error!".into()));

        r = r.with_alert(Alert::default());
        assert_eq!(r.alerts.len(), 1);

        r = r.with_alerts(vec![Alert::default(), Alert::default()]);
        assert_eq!(r.alerts.len(), 3);

        let r2 = RemediationResult::from_string(&String::from_utf8_lossy(&r.clone().write_to_bytes().unwrap()).into_owned());

        assert_eq!(r, r2);
    }
}

use alert::Alert;
use plugin;

// message RemediationResult {
//   required ResultType result = 1;
//   optional string error_msg = 2;
//   enum ResultType{
//     OK = 0;
//     ERR = 1;
//   }
//   repeated Alert alerts = 3;
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RemediationResult {
    pub result: Result<(), String>,
    pub alerts: Vec<Alert>,
}

impl RemediationResult {
    pub fn new() -> RemediationResult {
        RemediationResult {
            result: Ok(()),
            alerts: vec![],
        }
    }

    pub fn ok(mut self) -> Self {
        self.result = Ok(());
        self
    }

    pub fn err<T: Into<String>>(mut self, err: T) -> Self {
        self.result = Err(err.into());
        self
    }

    pub fn with_alert(mut self, alert: Alert) -> Self {
        self.alerts.push(alert);
        self
    }

    pub fn with_alerts(mut self, mut alerts: Vec<Alert>) -> Self {
        self.alerts.append(&mut alerts);
        self
    }

    pub fn write_to_bytes(self) -> Result<Vec<u8>, protobuf::ProtobufError> {
        let plugin_result: plugin::RemediationResult = self.into();

        plugin_result.write_to_bytes()
    }

    pub fn from_string(input: &String) -> RemediationResult {
        let r2 = parse_from_bytes::<plugin::RemediationResult>(input.as_bytes()).unwrap();
        r2.into()
    }

    pub fn from_bytes(input: &[u8]) -> RemediationResult {
        let r2 = parse_from_bytes::<plugin::RemediationResult>(input).unwrap();
        r2.into()
    }
}

impl<'a> From<&'a plugin::RemediationResult> for RemediationResult {
    fn from(result: &plugin::RemediationResult) -> RemediationResult {
        RemediationResult {
            result: match result.get_result() {
                plugin::RemediationResult_ResultType::OK => {
                    Ok(())
                }, plugin::RemediationResult_ResultType::ERR => {
                    Err(result.get_error_msg().to_string())
                }
            },
            alerts: result.get_alerts().iter()
                .map(|alert| alert.into())
                .collect(),
        }
    }
}

impl From<plugin::RemediationResult> for RemediationResult {
    fn from(result: plugin::RemediationResult) -> RemediationResult {
        (&result).into()
    }
}


impl From<RemediationResult> for plugin::RemediationResult {
    fn from(result: RemediationResult) -> plugin::RemediationResult {
        let mut a = plugin::RemediationResult::default();
        match result.result {
            Ok(()) => {
                a.set_result(plugin::RemediationResult_ResultType::OK);
            }
            Err(e) => {
                a.set_result(plugin::RemediationResult_ResultType::ERR);
                a.set_error_msg(e);
            }
        }
        let mut alerts = RepeatedField::default();
        for alert in result.alerts {
            alerts.push(alert.into());
        }
        a.set_alerts(alerts);
        a
    }
}

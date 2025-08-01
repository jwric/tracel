use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ExperimentMessage {
    MetricLog {
        name: String,
        epoch: usize,
        iteration: usize,
        value: f64,
        group: String,
    },
    Log(String),
    Error(String),
}

impl<S: Into<String> + Clone> From<S> for ExperimentMessage {
    fn from(msg: S) -> Self {
        let deser_msg: Result<ExperimentMessage, _> = serde_json::from_str(&msg.clone().into());
        match deser_msg {
            Ok(msg) => msg,
            Err(_) => ExperimentMessage::Error(format!("Invalid message: {}", msg.into())),
        }
    }
}

impl fmt::Display for ExperimentMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json_string = serde_json::to_string(self).expect("WsMessage should serialize to JSON");
        write!(f, "{json_string}")
    }
}

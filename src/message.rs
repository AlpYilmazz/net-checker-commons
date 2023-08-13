use pulsar::{DeserializeMessage, Payload, SerializeMessage};
use serde::{Deserialize, Serialize};

use crate::{MonitorConfiguration, MonitorTimingSettings};

#[derive(Serialize, Deserialize)]
pub enum MonitorMessage {
    Create(CreateMonitorMessage),
    Pause(PauseMonitorMessage),
    Resume(ResumeMonitorMessage),
    Kill(KillMonitorMessage),
}

impl SerializeMessage for MonitorMessage {
    fn serialize_message(input: Self) -> Result<pulsar::producer::Message, pulsar::Error> {
        let payload =
            serde_json::to_vec(&input).map_err(|e| pulsar::Error::Custom(e.to_string()))?;
        Ok(pulsar::producer::Message {
            payload,
            ..Default::default()
        })
    }
}

impl DeserializeMessage for MonitorMessage {
    type Output = Result<Self, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateMonitorMessage {
    pub monitor_id: String,
    pub timing: MonitorTimingSettings,
    pub monitor: MonitorConfiguration,
}

#[derive(Serialize, Deserialize)]
pub struct PauseMonitorMessage {
    pub monitor_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResumeMonitorMessage {
    pub monitor_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct KillMonitorMessage {
    pub monitor_id: String,
}

pub fn topic(act: &str, worker_id: &str) -> String {
    format!("persistent://public/net-monitor/{act}_{worker_id}")
}

use serde::{Deserialize, Serialize};

use crate::{MonitorConfiguration, MonitorTimingSettings};

#[derive(Serialize, Deserialize)]
pub struct CreateMonitorMessage {
    pub timing: MonitorTimingSettings,
    pub monitor: MonitorConfiguration,
}

#[derive(Serialize, Deserialize)]
pub enum ChangeMonitorStateMessage {
    Start { monitor_id: String },
    Stop { monitor_id: String },
}

#[derive(Serialize, Deserialize)]
pub struct DeleteMonitorMessage {
    pub monitor_id: String,
}

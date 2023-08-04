use bson::{oid::ObjectId, Bson, Document, doc};
use serde::{Deserialize, Serialize};

use crate::MonitorConfiguration;

#[derive(Serialize, Deserialize)]
pub struct Date(pub String);

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub _id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Worker {
    pub _id: String,
    pub name: String,
    pub city: String,
    pub country: String,
    pub region: String,
    pub work_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MonitorType {
    PING,
    HTTP,
    HTTPS,
    TCP,
    UDP,
}
impl From<MonitorType> for Bson {
    fn from(value: MonitorType) -> Self {
        match value {
            MonitorType::PING => "PING".into(),
            MonitorType::HTTP => "HTTP".into(),
            MonitorType::HTTPS => "HTTPS".into(),
            MonitorType::TCP => "TCP".into(),
            MonitorType::UDP => "UDP".into(),
        }
    }
}

impl From<&MonitorConfiguration> for MonitorType {
    fn from(value: &MonitorConfiguration) -> Self {
        match value {
            MonitorConfiguration::Ping(_) => Self::PING,
            MonitorConfiguration::Http(_) => Self::HTTP,
            MonitorConfiguration::Https(_) => Self::HTTPS,
            MonitorConfiguration::Tcp(_) => Self::TCP,
            MonitorConfiguration::Udp(_) => Self::UDP,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HealthMonitorConfiguration {
    pub _id: ObjectId,
    pub client_id: String,
    pub monitor_type: MonitorType,
    pub running: bool,
}

impl HealthMonitorConfiguration {
    pub fn as_insert_doc(client_id: &str, monitor_type: MonitorType) -> Document {
        doc! {
            "client_id": client_id,
            "monitor_type": monitor_type,
            "running": true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ReportType {
    HEALTHY,
    SLOW,
    TIMEOUT,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub report_type: ReportType,
    pub time: f32,
}

#[derive(Serialize, Deserialize)]
pub struct HealthCheckReport {
    _id: ObjectId,
    timestamp: Date,
    configuration_id: ObjectId,
    client_id: String,
    location_id: String,
    monitor_type: MonitorType,
    report: Report,
}

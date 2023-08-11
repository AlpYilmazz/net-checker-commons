use serde::{Deserialize, Serialize};

pub mod db;
pub mod message;

#[derive(Serialize, Deserialize)]
pub struct HealthMonitorRequest {
    pub geo_settings: GeoSettings,
    pub monitor: MonitorConfiguration,
}

#[derive(Serialize, Deserialize)]
pub struct StatusChangeRequest {
    pub running: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MonitorConfiguration {
    Ping(PingMonitorConfiguration),
    Http(HttpMonitorConfiguration),
    Https(HttpsMonitorConfiguration),
    Tcp(TcpMonitorConfiguration),
    Udp(UdpMonitorConfiguration),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PingMonitorConfiguration {
    pub host: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub auth: HttpAuth,
    pub custom_headers: Vec<HttpHeader>,
    pub body: Option<String>,
    pub response_expect: Vec<HttpResponseExpect>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpsMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub auth: HttpAuth,
    pub custom_headers: Vec<HttpHeader>,
    pub body: Option<String>,
    pub response_expect: Vec<HttpResponseExpect>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TcpMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub send: Option<String>,
    pub receive: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UdpMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub send: Option<String>,
    pub receive: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GeoSettings {
    pub locations: Vec<(GeoLocation, MonitorTimingSettings)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GeoLocation {
    Name(String),
    City(String),
    Country(String),
    Region(String),
}

/// 0 -- HEALTHY --> <self.healthy> -- SLOW --> <self.timeout> -- TIMEOUT -->
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MonitorTimingSettings {
    pub repeat_secs: u32,
    pub healthy: u32,
    pub timeout: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum HttpAuth {
    None,
    Bearer(String),
    Custom(String, String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum HttpResponseExpect {
    Any,
    Header(HttpHeader),
    Body(String),
}

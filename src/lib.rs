use serde::{Deserialize, Serialize};

pub mod db;
pub mod message;

#[derive(Serialize, Deserialize)]
pub struct HealthMonitorRequest {
    pub geo_settings: GeoSettings,
    pub timing_settings: MonitorTimingSettings,
    pub monitor: MonitorConfiguration,
}

#[derive(Serialize, Deserialize)]
pub enum MonitorConfiguration {
    Ping(PingMonitorConfiguration),
    Http(HttpMonitorConfiguration),
    Https(HttpsMonitorConfiguration),
    Tcp(TcpMonitorConfiguration),
    Udp(UdpMonitorConfiguration),
}

#[derive(Serialize, Deserialize)]
pub struct PingMonitorConfiguration {
    pub host: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub auth: HttpAuth,
    pub custom_headers: Vec<HttpHeader>,
    pub body: Option<String>,
    pub response_expect: Vec<HttpResponseExpect>,
}

#[derive(Serialize, Deserialize)]
pub struct HttpsMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub auth: HttpAuth,
    pub custom_headers: Vec<HttpHeader>,
    pub body: Option<String>,
    pub response_expect: Vec<HttpResponseExpect>,
}

#[derive(Serialize, Deserialize)]
pub struct TcpMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub send: Option<String>,
    pub receive: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UdpMonitorConfiguration {
    pub host: String,
    pub port: u32,
    pub send: Option<String>,
    pub receive: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GeoSettings {
    pub locations: Vec<GeoLocation>,
}

#[derive(Serialize, Deserialize)]
pub enum GeoLocation {
    Name(String),
    City(String),
    Country(String),
    Region(String),
}

/// 0 -- HEALTHY --> <self.healthy> -- SLOW --> <self.timeout> -- TIMEOUT -->
#[derive(Serialize, Deserialize)]
pub struct MonitorTimingSettings {
    pub repeat_secs: f32,
    pub healthy: f32,
    pub timeout: f32,
}

#[derive(Serialize, Deserialize)]
pub enum HttpAuth {
    None,
    Bearer(String),
    Custom(String, String),
}

#[derive(Serialize, Deserialize)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub enum HttpResponseExpect {
    Any,
    Header(HttpHeader),
    Body(String),
}

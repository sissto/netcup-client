use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsZone {
    pub name: String,
    pub ttl: String,
    pub serial: String,
    pub refresh: String,
    pub retry: String,
    pub expire: String,
    #[serde(rename(deserialize = "dnssecstatus"))]
    pub dns_sec_status: bool,
}

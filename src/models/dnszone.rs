use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DnsZone {
    pub name: String,
    pub ttl: String,
    pub serial: String,
    pub refresh: String,
    pub retry: String,
    pub expire: String,
    #[serde(rename = "dnssecstatus")]
    pub dns_sec_status: bool,
}

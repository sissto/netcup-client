use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsRecord {
    pub id: String,
    pub hostname: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub priority: String,
    pub destination: String,
    pub deleterecord: bool,
    pub state: String,
}

impl DnsRecord {
    pub fn new(hostname: &str, record_type: &str, destination: &str) -> Self {
        Self {
            id: String::default(),
            hostname: hostname.to_owned(),
            record_type: record_type.to_owned(),
            priority: String::default(),
            destination: destination.to_owned(),
            deleterecord: false,
            state: String::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsRecordSet {
    #[serde(rename = "dnsrecords")]
    pub records: Vec<DnsRecord>,
}

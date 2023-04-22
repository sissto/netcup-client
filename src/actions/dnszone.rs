use serde::Serialize;

use crate::prelude::DnsZone;

#[derive(Serialize)]
pub struct InfoDnsZoneAction {
    #[serde(rename(serialize = "customernumber"))]
    customer_no: i64,
    #[serde(rename(serialize = "apikey"))]
    api_key: String,
    #[serde(rename(serialize = "apisessionid"))]
    api_session_id: String,
    #[serde(rename(serialize = "domainname"))]
    domain_name: String,
}

impl InfoDnsZoneAction {
    pub fn new(customer_no: i64, api_key: &str, api_session_id: &str, domain_name: &str) -> Self {
        Self {
            customer_no,
            api_key: api_key.to_owned(),
            api_session_id: api_session_id.to_owned(),
            domain_name: domain_name.to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct UpdateDnsZoneAction {
    #[serde(rename(serialize = "customernumber"))]
    customer_no: i64,
    #[serde(rename(serialize = "apikey"))]
    api_key: String,
    #[serde(rename(serialize = "apisessionid"))]
    api_session_id: String,
    #[serde(rename(serialize = "domainname"))]
    domain_name: String,
    #[serde(rename(serialize = "dnszone"))]
    dns_zone: DnsZone,
}

impl UpdateDnsZoneAction {
    pub fn new(customer_no: i64, api_key: &str, api_session_id: &str, dns_zone: DnsZone) -> Self {
        Self {
            customer_no,
            api_key: api_key.to_owned(),
            api_session_id: api_session_id.to_owned(),
            domain_name: dns_zone.name.to_owned(),
            dns_zone,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{dnszone::*, request::RequestActionBuilder},
        prelude::DnsZone,
    };

    #[test]
    fn infodnszone_action_request() {
        let param = InfoDnsZoneAction::new(4711, "abc123", "session4711", "example.tld");
        let request = RequestActionBuilder::build("infodnszone", param);
        let json = serde_json::to_string(&request);
        assert_eq!("{\"action\":\"infodnszone\",\"param\":{\"customernumber\":4711,\"apikey\":\"abc123\",\"apisessionid\":\"session4711\",\"domainname\":\"example.tld\"}}", json.unwrap());
    }

    #[test]
    fn updatednszone_action_request() {
        let dns_zone = DnsZone {
            name: "example.tld".to_string(),
            ttl: "86400".to_string(),
            serial: "1234567".to_string(),
            refresh: "28800".to_string(),
            retry: "7200".to_string(),
            expire: "1209600".to_string(),
            dns_sec_status: false,
        };
        let param = UpdateDnsZoneAction::new(4711, "abc123", "session4711", dns_zone);
        let request = RequestActionBuilder::build("updatednszone", param);
        let json = serde_json::to_string(&request);
        assert_eq!("{\"action\":\"updatednszone\",\"param\":{\"customernumber\":4711,\"apikey\":\"abc123\",\"apisessionid\":\"session4711\",\"domainname\":\"example.tld\",\"dnszone\":{\"name\":\"example.tld\",\"ttl\":\"86400\",\"serial\":\"1234567\",\"refresh\":\"28800\",\"retry\":\"7200\",\"expire\":\"1209600\",\"dns_sec_status\":false}}}", json.unwrap());
    }
}

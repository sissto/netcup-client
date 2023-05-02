use serde::Serialize;

use crate::models::dnsrecord::DnsRecordSet;

#[derive(Serialize)]
pub struct InfoDnsRecordsAction {
    #[serde(rename(serialize = "customernumber"))]
    customer_no: u64,
    #[serde(rename(serialize = "apikey"))]
    api_key: String,
    #[serde(rename(serialize = "apisessionid"))]
    api_session_id: String,
    #[serde(rename(serialize = "domainname"))]
    domain_name: String,
}

impl InfoDnsRecordsAction {
    pub fn new(customer_no: u64, api_key: &str, api_session_id: &str, domain_name: &str) -> Self {
        Self {
            customer_no,
            api_key: api_key.to_owned(),
            api_session_id: api_session_id.to_owned(),
            domain_name: domain_name.to_owned(),
        }
    }
}

#[derive(Serialize)]
pub struct UpdateDnsRecordsAction {
    #[serde(rename(serialize = "customernumber"))]
    customer_no: u64,
    #[serde(rename(serialize = "apikey"))]
    api_key: String,
    #[serde(rename(serialize = "apisessionid"))]
    api_session_id: String,
    #[serde(rename(serialize = "domainname"))]
    domain_name: String,
    #[serde(rename(serialize = "dnsrecordset"))]
    dns_records: DnsRecordSet,
}

impl UpdateDnsRecordsAction {
    pub fn new(
        customer_no: u64,
        api_key: &str,
        api_session_id: &str,
        domain_name: &str,
        dns_records: DnsRecordSet,
    ) -> Self {
        Self {
            customer_no,
            api_key: api_key.to_owned(),
            api_session_id: api_session_id.to_owned(),
            domain_name: domain_name.to_owned(),
            dns_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{
            dnsrecord::{InfoDnsRecordsAction, UpdateDnsRecordsAction},
            request::RequestActionBuilder,
        },
        models::dnsrecord::DnsRecordSet,
        prelude::DnsRecord,
    };

    #[test]
    fn infodnsrecords_action_request() {
        let param = InfoDnsRecordsAction::new(4711, "abc123", "session4711", "example.tld");
        let request = RequestActionBuilder::build("infodnsrecords", param);
        let json = serde_json::to_string(&request);
        assert_eq!("{\"action\":\"infodnsrecords\",\"param\":{\"customernumber\":4711,\"apikey\":\"abc123\",\"apisessionid\":\"session4711\",\"domainname\":\"example.tld\"}}", json.unwrap());
    }

    #[test]
    fn updatednsrecords_action_request() {
        let records = vec![DnsRecord::new("@", "A", "127.0.0.1")];
        let param = UpdateDnsRecordsAction::new(
            4711,
            "abc123",
            "session4711",
            "example.tld",
            DnsRecordSet { records },
        );
        let request = RequestActionBuilder::build("updatednsrecords", param);
        let json = serde_json::to_string(&request);
        assert_eq!("{\"action\":\"updatednsrecords\",\"param\":{\"customernumber\":4711,\"apikey\":\"abc123\",\"apisessionid\":\"session4711\",\"domainname\":\"example.tld\",\"dnsrecordset\":{\"dnsrecords\":[{\"id\":\"\",\"hostname\":\"@\",\"type\":\"A\",\"priority\":\"\",\"destination\":\"127.0.0.1\",\"deleterecord\":false,\"state\":\"\"}]}}}", json.unwrap());
    }
}

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::{
    actions::{
        dnsrecord::*,
        dnszone::*,
        login::LoginAction,
        logout::LogoutAction,
        request::{RequestAction, RequestActionBuilder},
    },
    models::{
        dnsrecord::DnsRecordSet,
        login::LoginData,
        responses::{NCDataResponse, NCResponse},
    },
    prelude::{DnsRecord, DnsZone},
};

pub struct Unauthorized;
pub struct Authorized;

const BASE_URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";

pub struct NetcupClient<State = Unauthorized> {
    api_key: String,
    customer_no: u64,
    session_id: String,
    state: PhantomData<State>,
}

impl NetcupClient {
    pub fn new(api_key: &str, customer_no: u64) -> Self {
        NetcupClient {
            api_key: api_key.to_owned(),
            customer_no,
            session_id: String::default(),
            state: PhantomData,
        }
    }
}

impl<State> NetcupClient<State> {
    pub fn get_session_id(&self) -> String {
        self.session_id.to_string()
    }

    async fn send<Action>(&self, request: &RequestAction<Action>) -> Result<serde_json::Value>
    where
        Action: Serialize,
    {
        let response: serde_json::Value = reqwest::Client::new()
            .post(BASE_URL)
            .json(request)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    fn get_response(&self, value: &str) -> Result<NCResponse> {
        let data = serde_json::from_str::<NCResponse>(value).expect("JSON was not well-formatted");
        Ok(data)
    }

    fn get_response_data<'a, Data>(&self, value: &'a str) -> Result<NCDataResponse<Data>>
    where
        Data: Deserialize<'a>,
    {
        let result = serde_json::from_str::<NCDataResponse<Data>>(value);
        if result.is_err() {
            let err_result = self.get_response(value)?;
            bail!("{} - {}", err_result.short_message, err_result.long_message);
        }
        let data = result.expect("JSON was not well-formatted");
        Ok(data)
    }
}

impl NetcupClient<Unauthorized> {
    pub async fn login(self, api_password: &str) -> Result<NetcupClient<Authorized>> {
        let param = LoginAction::new(self.customer_no, &self.api_key, api_password);
        let request = RequestActionBuilder::build("login", param);

        let response = self.send::<LoginAction>(&request).await?;
        let data = self.get_response_data::<LoginData>(&response.to_string())?;

        let client = NetcupClient {
            api_key: self.api_key,
            customer_no: self.customer_no,
            session_id: data.data.api_session_id,
            state: PhantomData,
        };
        Ok(client)
    }
}

impl NetcupClient<Authorized> {
    pub async fn logout(self) -> Result<NetcupClient<Unauthorized>> {
        let param = LogoutAction::new(self.customer_no, &self.api_key, &self.session_id);
        let request = RequestActionBuilder::build("logout", param);

        let response = self.send::<LogoutAction>(&request).await?;
        let data = self.get_response(&response.to_string())?;

        if data.status.eq("error") {
            bail!("{} - {}", data.short_message, data.long_message);
        }

        let client = NetcupClient {
            api_key: self.api_key,
            customer_no: self.customer_no,
            session_id: String::default(),
            state: PhantomData,
        };
        Ok(client)
    }

    pub async fn get_dns_zone(&self, domain_name: &str) -> Result<DnsZone> {
        let param = InfoDnsZoneAction::new(
            self.customer_no,
            &self.api_key,
            &self.session_id,
            domain_name,
        );
        let request = RequestActionBuilder::build("infoDnsZone", param);

        let response = self.send::<InfoDnsZoneAction>(&request).await?;
        let data = self.get_response_data::<DnsZone>(&response.to_string())?;

        Ok(data.data)
    }

    pub async fn update_dns_zone(&self, dns_zone: DnsZone) -> Result<DnsZone> {
        let param =
            UpdateDnsZoneAction::new(self.customer_no, &self.api_key, &self.session_id, dns_zone);
        let request = RequestActionBuilder::build("updateDnsZone", param);

        let response = self.send::<UpdateDnsZoneAction>(&request).await?;
        let data = self.get_response_data::<DnsZone>(&response.to_string())?;

        Ok(data.data)
    }

    pub async fn get_dns_records(&self, domain_name: &str) -> Result<Vec<DnsRecord>> {
        let param = InfoDnsRecordsAction::new(
            self.customer_no,
            &self.api_key,
            &self.session_id,
            domain_name,
        );
        let request = RequestActionBuilder::build("infoDnsRecords", param);

        let response = self.send::<InfoDnsRecordsAction>(&request).await?;
        let data = self.get_response_data::<DnsRecordSet>(&response.to_string())?;

        Ok(data.data.records)
    }

    pub async fn update_dns_records(
        &self,
        domain_name: &str,
        records: Vec<DnsRecord>,
    ) -> Result<()> {
        if records.is_empty() {
            bail!("No DNS records provided.");
        } else if records.len() > 20 {
            bail!("More than 20 entries are currently not supported.")
        }

        let param = UpdateDnsRecordsAction::new(
            self.customer_no,
            &self.api_key,
            &self.session_id,
            domain_name,
            DnsRecordSet { records },
        );
        let request = RequestActionBuilder::build("updateDnsRecords", param);

        let response = self.send::<UpdateDnsRecordsAction>(&request).await?;
        let data = self.get_response(&response.to_string())?;

        if data.status.eq("error") {
            bail!("{} - {}", data.short_message, data.long_message);
        }

        Ok(())
    }
}

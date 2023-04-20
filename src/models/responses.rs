use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NCResponse {
    #[serde(rename(deserialize = "serverrequestid"))]
    pub server_request_id: String,
    #[serde(rename(deserialize = "clientrequestid"))]
    pub client_request_id: String,
    pub action: String,
    pub status: String,
    #[serde(rename(deserialize = "statuscode"))]
    pub status_code: i64,
    #[serde(rename(deserialize = "shortmessage"))]
    pub short_message: String,
    #[serde(rename(deserialize = "longmessage"))]
    pub long_message: String,
}

#[derive(Debug, Deserialize)]
pub struct NCDataResponse<Data> {
    #[serde(rename(deserialize = "responsedata"))]
    pub data: Data,
}

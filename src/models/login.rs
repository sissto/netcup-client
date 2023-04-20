use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginData {
    #[serde(rename(deserialize = "apisessionid"))]
    pub api_session_id: String,
}

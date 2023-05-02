use serde::Serialize;

#[derive(Serialize)]
pub struct LogoutAction {
    #[serde(rename(serialize = "customernumber"))]
    customer_no: u64,
    #[serde(rename(serialize = "apikey"))]
    api_key: String,
    #[serde(rename(serialize = "apisessionid"))]
    api_session_id: String,
}

impl LogoutAction {
    pub fn new(customer_no: u64, api_key: &str, api_session_id: &str) -> Self {
        Self {
            customer_no,
            api_key: api_key.to_owned(),
            api_session_id: api_session_id.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::{logout::LogoutAction, request::RequestActionBuilder};

    #[test]
    fn logout_action_request() {
        let param = LogoutAction::new(4711, "abc123", "session4711");
        let request = RequestActionBuilder::build("logout", param);
        let json = serde_json::to_string(&request);
        assert_eq!("{\"action\":\"logout\",\"param\":{\"customernumber\":4711,\"apikey\":\"abc123\",\"apisessionid\":\"session4711\"}}", json.unwrap());
    }
}

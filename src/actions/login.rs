use serde::Serialize;

#[derive(Serialize)]
pub struct LoginAction {
    #[serde(rename(serialize = "customernumber"))]
    customer_no: u64,
    #[serde(rename(serialize = "apikey"))]
    api_key: String,
    #[serde(rename(serialize = "apipassword"))]
    api_password: String,
}

impl LoginAction {
    pub fn new(customer_no: u64, api_key: &str, api_password: &str) -> Self {
        Self {
            customer_no,
            api_key: api_key.to_owned(),
            api_password: api_password.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::{login::LoginAction, request::RequestActionBuilder};

    #[test]
    fn login_action_request() {
        let param = LoginAction::new(4711, "abc123", "password123");
        let request = RequestActionBuilder::build("login", param);
        let json = serde_json::to_string(&request);
        assert_eq!("{\"action\":\"login\",\"param\":{\"customernumber\":4711,\"apikey\":\"abc123\",\"apipassword\":\"password123\"}}", json.unwrap());
    }
}

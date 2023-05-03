use anyhow::Result;
use netcup_client::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let customer_no = 4711;
    let api_key = "api_key";
    let api_password = "api_password";

    // login
    let client = NetcupClient::new(api_key, customer_no);
    let client = client.login(api_password).await?;
    println!("session id: {}", client.get_session_id());

    // logout
    let client = client.logout().await?;
    println!("session id: {}", client.get_session_id());
    Ok(())
}

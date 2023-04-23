use netcup_client::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let customer_no = 4711;
    let api_key = "api_key";
    let api_password = "api_password";

    // login
    let client = NetcupClient::new(api_key, customer_no);
    let client = client.login(api_password).await?;

    // get dns records
    let result = client.get_dns_records("example.tld").await?;
    println!("{:?}", result);

    // logout
    client.logout().await?;
    Ok(())
}

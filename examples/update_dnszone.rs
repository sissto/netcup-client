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

    // get dns zone
    let mut dns_zone = client.get_dns_zone("example.tld").await?;
    println!("{:?}", dns_zone);

    // change dns zone
    dns_zone.ttl = "86400".to_string();

    // update dns zone
    dns_zone = client.update_dns_zone(dns_zone).await?;
    println!("{:?}", dns_zone);

    // logout
    client.logout().await?;
    Ok(())
}

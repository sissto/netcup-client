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
    let mut dns_records = client.get_dns_records("example.tld").await?;
    println!("{:?}", dns_records);

    // get record to update by hostname
    let record = dns_records
        .iter_mut()
        .find(|r| r.hostname.eq("@"))
        .expect("DNS record not found");

    // change dns record
    record.destination = "127.0.0.1".to_string();

    // update dns records
    client
        .update_dns_records("example.tld", dns_records)
        .await?;

    // logout
    client.logout().await?;
    Ok(())
}

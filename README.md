# Netcup client

[![Cargo Build & Test](https://github.com/sissto/netcup-client/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/sissto/netcup-client/actions/workflows/rust.yml)

Client for the [netcup DNS API](https://www.netcup-wiki.de/wiki/DNS_API). It's not related to the netcup GmbH.

## Prerequisites

- Netcup account with at least one domain
- Api-key and api-password generated via netcup account management tool ([CCP](https://www.customercontrolpanel.de/))

## Getting started

```rust
    let customer_no = 4711;
    let api_key = "api_key";
    let api_password = "api_password";

    // login
    let client = NetcupClient::new(api_key, customer_no);
    let client = client.login(api_password).await?;

    // do your things here
    let records = client.get_dns_records("example.tld").await?;
    ...

    // logout
    client.logout().await?;
```

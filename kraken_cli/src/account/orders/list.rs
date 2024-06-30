use std::env;

use clap::ArgMatches;
use kraken_rest_client::Client;

pub async fn account_orders_list(_matches: &ArgMatches) -> anyhow::Result<()> {
    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET must be set");

    let client = Client::new(api_key, api_secret);

    let resp = client.get_open_orders().send().await;

    match resp {
        Ok(resp) => println!("{:?}", resp.open),
        Err(error) => eprintln!("{:?}", error),
    }

    Ok(())
}

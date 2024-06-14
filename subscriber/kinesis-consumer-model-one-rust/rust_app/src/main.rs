mod data_ops;
mod errors;
mod models;

use std::time::Duration;

use aws_config::{from_env, meta::region::RegionProviderChain};
use aws_lambda_events::kinesis::KinesisEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use models::ModelOne;
use momento::cache::configurations;
use momento::{CacheClient, CredentialProvider};
use tracing::{error, info};

use crate::data_ops::fetch_item;

async fn function_handler(
    cache_client: &CacheClient,
    ddb_client: &aws_sdk_dynamodb::Client,
    event: LambdaEvent<KinesisEvent>,
) -> Result<(), Error> {
    info!("Starting the loop ...");
    for e in event.payload.records {
        let mut model_one: ModelOne = e.into();
        info!("(ModelOne BEFORE)={:?}", model_one);

        let result = fetch_item(ddb_client, cache_client, model_one.read_location.clone()).await;
        match result {
            Ok(r) => {
                model_one.location = r;
                info!("(ModelOne AFTER)={:?}", model_one);
            }
            Err(e) => {
                error!("(Err)={:?}", e);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        .json()
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let region_provider = RegionProviderChain::default_provider();
    let config = from_env().region(region_provider).load().await;
    let client = aws_sdk_ssm::Client::new(&config);
    let ddb_client = aws_sdk_dynamodb::Client::new(&config);
    let parameter = client
        .get_parameter()
        .name("/keys/momento-pct-key")
        .send()
        .await?;

    let api_key = match parameter.parameter {
        Some(p) => p.value.unwrap(),
        None => panic!("Error with parameter"),
    };

    let cache_client = match CacheClient::builder()
        .default_ttl(Duration::from_secs(10))
        .configuration(configurations::Laptop::latest())
        .credential_provider(CredentialProvider::from_string(api_key).unwrap())
        .build()
    {
        Ok(c) => c,
        Err(_) => panic!("error with momento client"),
    };
    let shared_cache_client = &cache_client;
    let shared_ddb_client = &ddb_client;

    run(service_fn(
        move |event: LambdaEvent<KinesisEvent>| async move {
            function_handler(shared_cache_client, shared_ddb_client, event).await
        },
    ))
    .await
}

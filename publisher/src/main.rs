pub mod models;

use aws_config::{from_env, meta::region::RegionProviderChain};
use aws_sdk_kinesis::{primitives::Blob, Client};
use svix_ksuid::{Ksuid, KsuidLike};

use crate::models::ModelOne;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider();
    let config = from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let mut i = 0;



    while i < 1000 {
        let model_one = ModelOne::new(
            String::from("Model One"),
            vec![
                String::from("House"),
                String::from("Car"),
                String::from("Diner"),
            ],
        );

        let model_one_json = serde_json::to_string(&model_one);
        let model_one_blob = Blob::new(model_one_json.unwrap());
        let key = model_one.get_id();

        let result = client
            .put_record()
            .data(model_one_blob)
            .partition_key(key)
            .stream_name("kinesis-consumer-KinesisStream-sVfjh1gC2Nkd".to_string())
            .send()
            .await;

        match result {
            Ok(_) => {
                println!("Success!");
            }
            Err(_) => {
                println!("Error putting");
            }
        }

        i += 1;
    }
}

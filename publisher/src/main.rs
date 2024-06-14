pub mod models;

use aws_config::{from_env, meta::region::RegionProviderChain};
use aws_sdk_kinesis::{primitives::Blob, Client};
use tokio::task::JoinHandle;

use crate::models::ModelOne;

async fn thread_runner(client: Client) {
    let mut i = 0;
    while i < 10000 {
        let model_one = ModelOne::new(String::from("Model One"));

        let model_one_json = serde_json::to_string(&model_one);
        let model_one_blob = Blob::new(model_one_json.unwrap());
        let key = model_one.get_id();

        let result = client
            .put_record()
            .data(model_one_blob)
            .partition_key(key)
            .stream_name("kinesis-consumer-KinesisStream-Z1UZaKOMwWxA".to_string())
            .send()
            .await;

        match result {
            Ok(_) => {
                println!("Success!");
            }
            Err(e) => {
                println!("Error putting");
                println!("{:?}", e);
            }
        }

        i += 1;
    }
}

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider();
    let config = from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let mut loop_counter = 0;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    while loop_counter < 10 {
        let cloned_client = client.clone();
        let handle = tokio::spawn(async {
            thread_runner(cloned_client).await;
        });
        handles.push(handle);
        loop_counter += 1;
    }
    while let Some(h) = handles.pop() {
        h.await.unwrap();
    }
}

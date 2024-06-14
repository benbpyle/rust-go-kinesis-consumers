pub mod models;

use std::{env::VarError, i32};

use aws_config::{from_env, meta::region::RegionProviderChain};
use aws_sdk_kinesis::{primitives::Blob, Client};
use tokio::task::JoinHandle;

use crate::models::ModelOne;

/// Thread Runner
/// functions purpose is to take a client and execute a series of kinesis writes onto
/// a stream.  The number of writes is driven by an environment variable called RECORD_COUNT
/// In the absense of RECORD_COUNT, the function will default to 10
///
/// This function will be called from a thread, so there might be MANY of these called
/// at anyone time but they share no data
async fn thread_runner(client: Client) {
    // record count default to 10
    let record_count_var: Result<String, VarError> = std::env::var("RECORD_COUNT");
    let record_count: i32 = record_count_var
        .as_deref()
        .unwrap_or("10")
        .parse()
        .expect("RECORD_COUNT must be an int");

    // this is where it publishes.
    // RUN the SAM code in the publisher and take the Stream Name and put that in an environment
    // variable to make this work
    let kinesis_stream =
        std::env::var("KINESIS_STREAM_NAME").expect("KINESIS_STREAM_NAME is required");
    let mut i = 0;
    while i < record_count {
        let model_one = ModelOne::new(String::from("Model One"));

        // create a new model in the loop and push into kinesis
        let model_one_json = serde_json::to_string(&model_one);
        let model_one_blob = Blob::new(model_one_json.unwrap());
        let key = model_one.get_id();

        let result = client
            .put_record()
            .data(model_one_blob)
            .partition_key(key)
            .stream_name(kinesis_stream.to_string())
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

    // THREAD_COUNT defaults to 1 but can be changed to support multiple threads that'll execute
    // the thread_runner function as many times as defined in the RECORD_COUNT
    let thread_count_var: Result<String, VarError> = std::env::var("THREAD_COUNT");
    let thread_count: i32 = thread_count_var
        .as_deref()
        .unwrap_or("1")
        .parse()
        .expect("THREAD_COUNT must be an int");
    while loop_counter < thread_count {
        // create as many threads as defined
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

use aws_lambda_events::kinesis::KinesisEventRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelOne {
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "location"))]
    #[serde(skip_serializing)]
    pub read_location: String,
    #[serde(skip_deserializing)]
    pub location: Option<CacheModel>,
    pub write_time: DateTime<Utc>,
    pub read_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelTwo {
    id: String,
    name: String,
    scores: Vec<i32>,
    write_time: DateTime<Utc>,
    read_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheModel {
    pub location: String,
    pub description: String,
    pub notes: String,
}

impl From<KinesisEventRecord> for ModelOne {
    fn from(value: KinesisEventRecord) -> ModelOne {
        let mut model: ModelOne = serde_json::from_slice(value.kinesis.data.0.as_slice()).unwrap();
        model.read_time = Some(Utc::now());
        model
    }
}

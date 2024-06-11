use chrono::{DateTime, Utc};
use serde::Serialize;
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelOne {
    id: String,
    name: String,
    locations: Vec<String>,
    write_time: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelTwo {
    id: String,
    name: String,
    scores: Vec<i32>,
    write_time: DateTime<Utc>,
}

impl ModelOne {
    pub fn new(name: String, locations: Vec<String>) -> ModelOne {
        let model_one_id = Ksuid::new(None, None);

        ModelOne {
            locations,
            name,
            id: model_one_id.to_string(),
            write_time: Utc::now(),
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

impl ModelTwo {
    pub fn new(name: String, scores: Vec<i32>) -> ModelTwo {
        let model_two_id = Ksuid::new(None, None);

        ModelTwo {
            scores,
            name,
            id: model_two_id.to_string(),
            write_time: Utc::now(),
        }
    }
}

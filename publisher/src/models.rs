use serde::{Deserialize, Serialize};
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelOne {
    id: String,
    name: String,
    locations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelTwo {
    id: String,
    name: String,
    scores: Vec<i32>,
}

impl ModelOne {
    pub fn new(name: String, locations: Vec<String>) -> ModelOne {
        let model_one_id = Ksuid::new(None, None);

        ModelOne {
            locations,
            name,
            id: model_one_id.to_string(),
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
        }
    }
}

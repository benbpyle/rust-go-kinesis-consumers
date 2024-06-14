use chrono::{DateTime, Utc};
use rand::Rng;
use serde::Serialize;
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelOne {
    id: String,
    name: String,
    location: String,
    write_time: DateTime<Utc>,
}

impl ModelOne {
    pub fn new(name: String) -> ModelOne {
        let model_one_id = Ksuid::new(None, None);

        let locations = [
            String::from("House"),
            String::from("Car"),
            String::from("Diner"),
        ];
        let random_string_index: usize = rand::thread_rng().gen_range(0..locations.len());
        let s = locations[random_string_index].clone();
        ModelOne {
            location: s,
            name,
            id: model_one_id.to_string(),
            write_time: Utc::now(),
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
}

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

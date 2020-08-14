use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct HeapDump {
    snapshot: Value,
    pub nodes: Vec<u32>,
    pub strings: Vec<String>,
}

#[derive(std::fmt::Debug)]
pub struct Stats {
    pub total: usize,
    pub categories: Vec<(String, usize)>,
}

pub fn get_statistics() -> Stats {
    let vec: Vec<(String, usize)> =
        vec![(String::from("code"), 123), (String::from("strings"), 456)];

    // vec.push();
    // vec.push();

    let stats = Stats {
        total: 123,
        categories: vec,
    };

    stats
}

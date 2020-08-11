use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::env;
use std::fs;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct HeapDump {
    snapshot: Value,
    nodes: Vec<u32>,
    strings: Vec<String>,
}

fn run(file: &std::string::String) -> Result<HeapDump> {
    let data = fs::read_to_string(file).expect("NOPE");

    let heapdump: HeapDump = serde_json::from_str(&data)?;

    Ok(heapdump)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let heapdump = run(file).expect("?");

    println!("Number of nodes {}", heapdump.nodes.len());
    println!("Number of strings {}", heapdump.strings.len());

    let hello = warp::any().map(|| "Hello");
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

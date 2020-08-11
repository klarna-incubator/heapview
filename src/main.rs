use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize)]
struct HeapDump {
    snapshot: Value,
    nodes: Vec<u32>,
    strings: Vec<String>,
}

fn run(file: &std::string::String) -> Result<()> {
    let data = fs::read_to_string(file).expect("NOPE");

    let p: HeapDump = serde_json::from_str(&data)?;

    println!("Number of nodes {}", p.nodes.len());
    println!("Number of strings {}", p.strings.len());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    run(file).unwrap()
}

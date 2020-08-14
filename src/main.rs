use heapview::analyzer::{get_statistics, HeapDump};
use heapview::backend::create_server;
use serde_json::Result;
use std::env;
use std::fs;

fn run(file: &std::string::String) -> Result<HeapDump> {
    let data = fs::read_to_string(file).expect("NOPE");

    let heapdump: HeapDump = serde_json::from_str(&data)?;

    Ok(heapdump)
}

// #[tokio::main]
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let heapdump = run(file).expect("?");

    let stats = get_statistics(heapdump);
        fs::write(
            "/tmp/stats.json",
            serde_json::ser::to_string(&stats).expect("Failed to serialize the stats"),
        )
        .expect("Unable to write file");


    create_server("127.0.0.1:3000".to_string())
}

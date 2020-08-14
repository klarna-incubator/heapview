use heapview::analyzer::{get_statistics, parse_heapdump};
use heapview::backend::create_server;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let data = fs::read_to_string(file).expect("Failed to read file");
    let heapdump = parse_heapdump(data).expect("Failed to parse heapdump");
    let stats = get_statistics(heapdump);
    fs::write(
        "/tmp/stats.json",
        serde_json::ser::to_string(&stats).expect("Failed to serialize the stats"),
    )
    .expect("Unable to write file");

    create_server("127.0.0.1:3000".to_string())
}

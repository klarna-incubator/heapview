use std::env;
use std::fs;

use heapview::analyzer::{get_statistics, parse_heapdump};
use heapview::backend::create_server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let data = fs::read_to_string(file).expect("Failed to read file");
    let heapdump = parse_heapdump(data).expect("Failed to parse heapdump");
    let stats = get_statistics(heapdump);

    create_server("127.0.0.1:3000".to_string(), stats)
}

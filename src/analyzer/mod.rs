use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
enum NodeType {
    Hidden,
    Array,
    String,
    Object,
    Code,
    Closure,
    Regexp,
    Number,
    Native,
    Synthetic,
    ConcatenatedString,
    SlicedString,
    Symbol,
    Bigint,
    Uknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    meta: Value,
    node_count: usize,
    edge_count: usize,
    trace_function_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeapDump {
    pub snapshot: Snapshot,
    pub nodes: Vec<usize>,
    pub strings: Vec<String>,
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    name: usize,
    id: usize,
    self_size: usize,
    edge_count: usize,
    trace_node_id: usize,
}

#[derive(Debug)]
pub struct Stats {
    pub total: usize,
    pub categories: Vec<(String, usize)>,
    pub nodes: Vec<Node>,
}

fn get_node_type(type_number: usize) -> NodeType {
    match type_number {
        0 => NodeType::Hidden,
        1 => NodeType::Array,
        2 => NodeType::String,
        3 => NodeType::Object,
        4 => NodeType::Code,
        5 => NodeType::Closure,
        6 => NodeType::Regexp,
        7 => NodeType::Number,
        8 => NodeType::Native,
        9 => NodeType::Synthetic,
        10 => NodeType::ConcatenatedString,
        11 => NodeType::SlicedString,
        12 => NodeType::Symbol,
        13 => NodeType::Bigint,
        _ => NodeType::Uknown,
    }
}

pub fn get_statistics(heapdump: HeapDump) -> Stats {
    // let nodes: =
    let vec: Vec<(String, usize)> =
        vec![(String::from("code"), 123), (String::from("strings"), 456)];

    let mut nodes: Vec<Node> = vec![];
    for node_values in heapdump.nodes.chunks(6) {
        let node = Node {
            node_type: get_node_type(node_values[0]),
            name: node_values[1],
            id: node_values[2],
            self_size: node_values[3],
            edge_count: node_values[4],
            trace_node_id: node_values[5],
        };
        nodes.push(node)
    }
    // vec.push();
    // vec.push();

    let stats = Stats {
        total: 123,
        categories: vec,
        nodes: nodes,
    };

    stats
}

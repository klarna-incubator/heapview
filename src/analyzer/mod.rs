use gotham::state::StateData;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub enum NodeType {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snapshot {
    meta: Value,
    node_count: usize,
    edge_count: usize,
    trace_function_count: usize,
}

#[derive(Clone, StateData, Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize)]
pub struct Stats {
    pub total: usize,
    pub categories: HashMap<NodeType, usize>,
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
    let mut nodes: Vec<Node> = vec![];

    let mut stats = Stats {
        total: 0,
        categories: HashMap::new(),
    };

    for node_values in heapdump.nodes.chunks(6) {
        let node_type = get_node_type(node_values[0]);
        let node = Node {
            node_type: node_type.clone(),
            name: node_values[1],
            id: node_values[2],
            self_size: node_values[3],
            edge_count: node_values[4],
            trace_node_id: node_values[5],
        };

        let category_total_size = stats.categories.entry(node_type).or_insert(node.self_size);
        *category_total_size += node.self_size;

        stats.total += node.self_size;

        nodes.push(node)
    }

    println!("Stats {:?}", stats);

    stats
}

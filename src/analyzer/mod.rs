use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::collections::HashMap;

#[serde(rename_all = "camelCase")]
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

impl NodeType {
    fn get_node_type(type_number: usize) -> Self {
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
}

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub enum EdgeType {
    Context,
    Element,
    Property,
    Internal,
    Hidden,
    Shortcut,
    Weak,
    Unknown,
}

impl EdgeType {
    fn get_edge_type(type_number: usize) -> Self {
        match type_number {
            0 => EdgeType::Context,
            1 => EdgeType::Element,
            2 => EdgeType::Property,
            3 => EdgeType::Internal,
            4 => EdgeType::Hidden,
            5 => EdgeType::Shortcut,
            6 => EdgeType::Weak,
            _ => EdgeType::Unknown,
        }
    }
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
    pub edges: Vec<usize>,
    pub strings: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    node_type: NodeType,
    name: usize,
    id: usize,
    self_size: usize,
    edge_count: usize,
    trace_node_id: usize,
    // edges: Vec<Edge>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Edge {
    edge_type: EdgeType,
    from: usize,
    to: usize,
}

#[serde(rename_all = "camelCase")]
#[derive(Eq, PartialEq, Debug, Serialize)]
pub struct Stats {
    pub total: usize,
    pub categories: HashMap<NodeType, usize>,
}

pub fn get_statistics(heapdump: HeapDump) -> Stats {
    process_heapump(heapdump).0
}

pub fn get_nodes(heapdump: HeapDump) -> Vec<Node> {
    process_heapump(heapdump).1
}

pub fn get_edges(heapdump: HeapDump) -> Vec<Edge> {
    process_heapump(heapdump).2
}

pub fn process_heapump(heapdump: HeapDump) -> (Stats, Vec<Node>, Vec<Edge>) {
    let mut nodes: Vec<Node> = vec![];
    let mut edges: Vec<Edge> = vec![];

    let mut stats = Stats {
        total: 0,
        categories: HashMap::new(),
    };

    for node_values in heapdump.nodes.chunks(6) {
        let node_type = NodeType::get_node_type(node_values[0]);
        let node = Node {
            node_type: node_type.clone(),
            name: node_values[1],
            id: node_values[2],
            self_size: node_values[3],
            edge_count: node_values[4],
            trace_node_id: node_values[5],
        };

        let category_total_size = stats.categories.entry(node_type).or_insert(0);
        *category_total_size += node.self_size;

        stats.total += node.self_size;

        nodes.push(node)
    }

    for values in heapdump.edges.chunks(3) {
        let edge = Edge {
            edge_type: EdgeType::get_edge_type(values[0]),
            from: values[1],
            to: values[2],
        };

        edges.push(edge)
    }

    (stats, nodes, edges)
}

pub fn parse_heapdump(data: String) -> Result<HeapDump> {
    let heapdump: HeapDump = serde_json::from_str(&data)?;

    Ok(heapdump)
}

#[cfg(test)]
mod test {
    use super::{parse_heapdump, process_heapump, Edge, EdgeType, Node, NodeType, Stats};
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_process_heapump() {
        let data =
            fs::read_to_string("src/analyzer/fixtures/simple.json").expect("Failed to read file");
        let heapdump = parse_heapdump(data).expect("Failed to parse heapdump");

        let mut categories = HashMap::new();
        categories.insert(NodeType::Array, 1);
        categories.insert(NodeType::String, 2);
        categories.insert(NodeType::Object, 3);

        let (stats, nodes, edges) = process_heapump(heapdump);

        assert_eq!(
            stats,
            Stats {
                categories: categories,
                total: 6
            }
        );

        assert_eq!(
            nodes,
            vec![
                Node {
                    node_type: NodeType::Array,
                    name: 1,
                    id: 1,
                    self_size: 1,
                    edge_count: 2,
                    trace_node_id: 33333
                },
                Node {
                    node_type: NodeType::String,
                    name: 2,
                    id: 2,
                    self_size: 2,
                    edge_count: 1,
                    trace_node_id: 33333
                },
                Node {
                    node_type: NodeType::Object,
                    name: 3,
                    id: 3,
                    self_size: 3,
                    edge_count: 0,
                    trace_node_id: 33333
                }
            ]
        );

        assert_eq!(
            edges,
            vec![
                Edge {
                    edge_type: EdgeType::Element,
                    from: 1,
                    to: 2
                },
                Edge {
                    edge_type: EdgeType::Property,
                    from: 1,
                    to: 3
                },
                Edge {
                    edge_type: EdgeType::Internal,
                    from: 2,
                    to: 3
                }
            ]
        );
    }
}

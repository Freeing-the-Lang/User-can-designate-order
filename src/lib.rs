pub mod parser;
pub mod order_graph;

use order_graph::OrderGraph;

pub fn apply_order(
    auto_graph: Vec<String>,
    order_file: &str,
) -> anyhow::Result<Vec<String>> {
    let order = parser::parse_mmo(order_file)?;
    let merged = OrderGraph::merge(auto_graph, order)?;
    Ok(merged)
}

use anyhow::*;
use crate::parser::MmoEntry;

pub struct OrderGraph;

impl OrderGraph {
    pub fn merge(
        auto: Vec<String>,
        order: Vec<MmoEntry>,
    ) -> Result<Vec<String>> {

        let mut final_order = Vec::new();

        for entry in order {
            match entry {
                MmoEntry::Node(n) => {
                    final_order.push(n);
                }
                MmoEntry::Ignore(_) => { /* 나중에 필터링 */ }
                MmoEntry::Edge{..} => { /* DAG 처리: todo */ }
                MmoEntry::Group { nodes, .. } => {
                    final_order.extend(nodes);
                }
            }
        }

        // 자동 그래프의 나머지 노드 추가
        for n in auto {
            if !final_order.contains(&n) {
                final_order.push(n);
            }
        }

        Ok(final_order)
    }
}

use anyhow::*;

#[derive(Debug)]
pub enum MmoEntry {
    Node(String),
    Ignore(String),
    Edge { from: String, to: String },
    Group { name: String, nodes: Vec<String> },
}

pub fn parse_mmo(path: &str) -> Result<Vec<MmoEntry>> {
    let text = std::fs::read_to_string(path)?;
    let mut result = Vec::new();
    let mut current_group: Option<(String, Vec<String>)> = None;

    for line in text.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with("[group ") {
            let name = line.trim_matches(|c| c == '[' || c == ']').replace("group ", "");
            current_group = Some((name, Vec::new()));
            continue;
        }

        if line == "]" {
            if let Some((name, list)) = current_group.take() {
                result.push(MmoEntry::Group { name, nodes: list });
            }
            continue;
        }

        if let Some((_, ref mut list)) = current_group {
            list.push(line.to_string());
            continue;
        }

        if line.starts_with("ignore:") {
            let node = line.replace("ignore:", "").trim().to_string();
            result.push(MmoEntry::Ignore(node));
            continue;
        }

        if line.contains("->") {
            let sp: Vec<_> = line.split("->").collect();
            let from = sp[0].trim().to_string();
            let to = sp[1].trim().to_string();
            result.push(MmoEntry::Edge { from, to });
            continue;
        }

        result.push(MmoEntry::Node(line.to_string()));
    }

    Ok(result)
}

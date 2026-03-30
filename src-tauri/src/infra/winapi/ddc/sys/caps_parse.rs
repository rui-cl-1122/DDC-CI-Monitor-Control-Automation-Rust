use crate::infra::winapi::ddc::sys::error::CapabilitiesParseError;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Atom(String),
    List(Vec<Node>),
}

pub type Sections = HashMap<String, Vec<Option<Vec<Node>>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitiesParsed {
    pub meta: HashMap<String, String>,
    pub cmds: Vec<u8>,
    pub vcp_codes: Vec<u8>,
    pub vcp_subs: HashMap<u8, Vec<u8>>,
    pub sections: Sections,
}

type ParseResult<T> = std::result::Result<T, CapabilitiesParseError>;

fn tokenize(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }

    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c == '(' || c == ')' {
            out.push(c.to_string());
            i += 1;
            continue;
        }

        let start = i;
        while i < chars.len()
            && !chars[i].is_whitespace()
            && chars[i] != '('
            && chars[i] != ')'
        {
            i += 1;
        }

        out.push(chars[start..i].iter().collect());
    }

    out
}

fn parse_group(tokens: &[String], mut idx: usize) -> ParseResult<(Vec<Node>, usize)> {
    if idx >= tokens.len() || tokens[idx] != "(" {
        return Err(CapabilitiesParseError::new(format!(
            "Expected '(' at token index {idx}"
        )));
    }
    idx += 1;

    let mut items = Vec::new();

    loop {
        if idx >= tokens.len() {
            return Err(CapabilitiesParseError::new(
                "Unexpected end while parsing group",
            ));
        }

        match tokens[idx].as_str() {
            ")" => {
                idx += 1;
                break;
            }
            "(" => {
                let (sub, next) = parse_group(tokens, idx)?;
                items.push(Node::List(sub));
                idx = next;
            }
            atom => {
                items.push(Node::Atom(atom.to_string()));
                idx += 1;
            }
        }
    }

    Ok((items, idx))
}

fn to_top_sections(root_items: &[Node]) -> Sections {
    let mut sec: Sections = HashMap::new();
    let mut i = 0;

    while i < root_items.len() {
        let key = match &root_items[i] {
            Node::Atom(s) => s.trim().to_ascii_lowercase(),
            Node::List(_) => {
                i += 1;
                continue;
            }
        };

        let mut val: Option<Vec<Node>> = None;

        if i + 1 < root_items.len() {
            if let Node::List(xs) = &root_items[i + 1] {
                val = Some(xs.clone());
                i += 2;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }

        sec.entry(key).or_default().push(val);
    }

    sec
}

fn is_hex_byte(tok: &str) -> bool {
    !tok.is_empty() && tok.len() <= 2 && tok.chars().all(|c| c.is_ascii_hexdigit())
}

fn parse_hex_byte(tok: &str) -> Option<u8> {
    if !is_hex_byte(tok) {
        return None;
    }
    u8::from_str_radix(tok, 16).ok()
}

fn flatten_atoms(nodes: &[Node], out: &mut Vec<String>) {
    for node in nodes {
        match node {
            Node::Atom(s) => out.push(s.clone()),
            Node::List(xs) => flatten_atoms(xs, out),
        }
    }
}

fn unique_keep_order<T>(items: impl IntoIterator<Item = T>) -> Vec<T>
where
    T: Eq + std::hash::Hash + Copy,
{
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for x in items {
        if seen.insert(x) {
            out.push(x);
        }
    }

    out
}

fn extract_cmds(sections: &Sections) -> Vec<u8> {
    for key in ["cmds", "cmdlist", "cmd"] {
        if let Some(vals) = sections.get(key) {
            let mut tokens = Vec::new();

            for value in vals {
                if let Some(nodes) = value {
                    flatten_atoms(nodes, &mut tokens);
                }
            }

            return unique_keep_order(tokens.into_iter().filter_map(|t| parse_hex_byte(&t)));
        }
    }

    Vec::new()
}

fn extract_vcp(sections: &Sections) -> (Vec<u8>, HashMap<u8, Vec<u8>>) {
    let Some(values) = sections.get("vcp") else {
        return (Vec::new(), HashMap::new());
    };

    let mut parents = Vec::new();
    let mut subs = HashMap::<u8, Vec<u8>>::new();

    for value in values {
        let Some(items) = value else {
            continue;
        };

        let mut i = 0;
        while i < items.len() {
            match &items[i] {
                Node::Atom(atom) => {
                    if let Some(parent) = parse_hex_byte(atom) {
                        parents.push(parent);

                        if i + 1 < items.len() {
                            if let Node::List(child_nodes) = &items[i + 1] {
                                let mut child_atoms = Vec::new();
                                flatten_atoms(child_nodes, &mut child_atoms);

                                let child_codes = unique_keep_order(
                                    child_atoms
                                        .into_iter()
                                        .filter_map(|x| parse_hex_byte(&x)),
                                );

                                if !child_codes.is_empty() {
                                    subs.insert(parent, child_codes);
                                }

                                i += 2;
                                continue;
                            }
                        }
                    }

                    i += 1;
                }
                Node::List(_) => {
                    i += 1;
                }
            }
        }
    }

    (unique_keep_order(parents), subs)
}

fn extract_meta(sections: &Sections) -> HashMap<String, String> {
    let mut meta = HashMap::new();

    for (key, values) in sections {
        if matches!(key.as_str(), "cmds" | "cmdlist" | "cmd" | "vcp") {
            continue;
        }

        for value in values {
            let Some(nodes) = value else {
                continue;
            };

            let mut atoms = Vec::new();
            flatten_atoms(nodes, &mut atoms);
            atoms.retain(|x| !x.is_empty());

            if !atoms.is_empty() {
                meta.insert(key.clone(), atoms.join(" "));
                break;
            }
        }
    }

    meta
}

pub fn parse_capabilities(raw: &str) -> ParseResult<CapabilitiesParsed> {
    let mut s = raw.trim().to_string();

    if s.is_empty() {
        return Ok(CapabilitiesParsed {
            meta: HashMap::new(),
            cmds: Vec::new(),
            vcp_codes: Vec::new(),
            vcp_subs: HashMap::new(),
            sections: HashMap::new(),
        });
    }

    if let Some(first) = s.find('(') {
        s = s[first..].to_string();
    }
    if let Some(last) = s.rfind(')') {
        s = s[..=last].to_string();
    }

    let tokens = tokenize(&s);
    if tokens.is_empty() {
        return Ok(CapabilitiesParsed {
            meta: HashMap::new(),
            cmds: Vec::new(),
            vcp_codes: Vec::new(),
            vcp_subs: HashMap::new(),
            sections: HashMap::new(),
        });
    }

    if tokens.first().map(String::as_str) != Some("(") {
        return Err(CapabilitiesParseError::new(
            "Capabilities string does not start with '(' after normalization",
        ));
    }

    let (root_items, idx) = parse_group(&tokens, 0)?;
    if idx != tokens.len() {
        return Err(CapabilitiesParseError::new(format!(
            "Trailing tokens remain: idx={idx} len={}",
            tokens.len()
        )));
    }

    let sections = to_top_sections(&root_items);
    let cmds = extract_cmds(&sections);
    let (vcp_codes, vcp_subs) = extract_vcp(&sections);
    let meta = extract_meta(&sections);

    Ok(CapabilitiesParsed {
        meta,
        cmds,
        vcp_codes,
        vcp_subs,
        sections,
    })
}
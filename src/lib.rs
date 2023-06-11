use html_parser::{Dom, Element, Node};

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

pub fn parse(html: &str) -> Result<String, Error> {
    let dom = Dom::parse(html).map_err(|e| Error::ParseError(format!("{e}")))?;
    let mut res = vec![];
    for c in dom.children {
        res.push(node_parser(c))
    }

    Ok(res.join(""))
}
fn node_parser(node: Node) -> String {
    match node {
        html_parser::Node::Text(t) => format!("{t:?}"),
        html_parser::Node::Comment(_) => "".to_string(),
        html_parser::Node::Element(e) => element_parser(e),
    }
}

fn element_parser(element: Element) -> String {
    let mut out = format!("{} {{ ", element.name);
    if let Some(id) = element.id {
        let res = format!("id: \"{id}\", ");
        out.push_str(&res);
    }

    for (atbr, val) in &element.attributes {
        let res = if let Some(v) = val {
            format!("{atbr}: \"{v}\"")
        } else {
            format!("{atbr},")
        };
        out.push_str(&res);
    }

    if !element.classes.is_empty() {
        let res = format!("class: \"{}\",", element.classes.join(" "));
        out.push_str(&res);
    }

    let res = element
        .children
        .into_iter()
        .map(node_parser)
        .collect::<Vec<_>>()
        .join(" ");
    out.push_str(&res);

    out.push('}');
    out
}

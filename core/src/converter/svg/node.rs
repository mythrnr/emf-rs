use crate::imports::*;

#[derive(Clone, Debug)]
pub struct Node {
    typ: NodeType,
    inner: Vec<Node>,
    attrs: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
enum NodeType {
    Node(String),
    Text(String),
}

#[allow(clippy::needless_pass_by_value)]
impl Node {
    pub fn new(name: impl ToString) -> Self {
        Self {
            typ: NodeType::Node(name.to_string()),
            inner: vec![],
            attrs: BTreeMap::new(),
        }
    }

    pub fn new_text(value: impl ToString) -> Self {
        Self {
            typ: NodeType::Text(value.to_string()),
            inner: vec![],
            attrs: BTreeMap::new(),
        }
    }

    pub fn add(mut self, node: Node) -> Self {
        if matches!(self.typ, NodeType::Node(_)) {
            self.inner.push(node);
        }

        self
    }

    pub fn set(mut self, name: impl ToString, value: impl ToString) -> Self {
        if matches!(self.typ, NodeType::Node(_)) {
            self.attrs.insert(name.to_string(), value.to_string());
        }

        self
    }

    fn escape_text(value: impl ToString) -> String {
        let s = value.to_string();
        let mut out = String::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                // The only control characters XML 1.0 permits are
                // #x9 / #xA / #xD; strip the rest because they would
                // produce invalid SVG.
                '\x00'..='\x08' | '\x0B' | '\x0C' | '\x0E'..='\x1F' => {}
                _ => {
                    // Strip Unicode noncharacters that XML 1.0 forbids:
                    // U+FDD0-U+FDEF and U+xFFFE / U+xFFFF in each plane.
                    let code = c as u32;
                    let is_nonchar = (0xFDD0..=0xFDEF).contains(&code)
                        || (code & 0xFFFE) == 0xFFFE;

                    if !is_nonchar {
                        out.push(c);
                    }
                }
            }
        }

        out
    }

    fn escape_attr(value: impl ToString) -> String {
        Self::escape_text(value).replace('"', "&quot;").replace('\'', "&apos;")
    }
}

impl core::fmt::Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.typ {
            NodeType::Node(name) => {
                write!(f, "<{name}")?;

                for (k, v) in &self.attrs {
                    write!(f, r#" {k}="{}""#, Self::escape_attr(v))?;
                }

                write!(f, ">")?;

                for child in &self.inner {
                    write!(f, "{child}")?;
                }

                write!(f, "</{name}>")
            }
            NodeType::Text(value) => {
                write!(f, "{}", Self::escape_text(value))
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Data(String);

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push_command(&mut self, cmd: char, param: &Parameters) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }

        self.0.push(cmd);
        self.0.push(' ');
        self.0.push_str(&param.0);
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataClosePathCommand
    pub fn close(mut self) -> Self {
        if !self.0.is_empty() {
            self.0.push(' ');
        }

        self.0.push('Z');
        self
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    pub fn curve_to(mut self, param: impl Into<Parameters>) -> Self {
        self.push_command('C', &param.into());
        self
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    // Reserved for future Arc-family record support.
    #[allow(dead_code)]
    pub fn elliptical_arc_to(mut self, param: impl Into<Parameters>) -> Self {
        self.push_command('A', &param.into());
        self
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    pub fn line_to(mut self, param: impl Into<Parameters>) -> Self {
        self.push_command('L', &param.into());
        self
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataMovetoCommands
    pub fn move_to(mut self, param: impl Into<Parameters>) -> Self {
        self.push_command('M', &param.into());
        self
    }
}

impl core::fmt::Display for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Parameters(String);

impl From<String> for Parameters {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for Parameters {
    fn from(v: &str) -> Self {
        Self(v.to_owned())
    }
}

impl From<Vec<String>> for Parameters {
    fn from(v: Vec<String>) -> Self {
        Self(v.join(" "))
    }
}

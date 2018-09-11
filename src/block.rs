use std::fmt;

/// Represents a segment of a line, potentially containing another block
enum LineSegment {
    Content(String),
    Block(Block),
}

impl LineSegment {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineSegment::Content(s) => write!(f, "{}", s),
            LineSegment::Block(b) => write!(f, "[Recurse block]"),
        }
    }
}

/// Represents a single line inside a block of text
struct Line(Vec<LineSegment>);

impl<T: Into<String>> From<T> for Line {
    fn from(v: T) -> Self {
        Line(vec![LineSegment::Content(v.into())])
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0[0])
    }
}

struct Block(Vec<Line>);

impl<T: Into<String>> From<T> for Block {
    fn from(v: T) -> Self {
        Block(vec![Line::from(v.into())])
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0[0].0[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_a_block() {
        let result = GrammarParser::parse(Rule::file, TEST_TEMPLATE);
        let file: File = result.unwrap().next().unwrap().into();
        assert_eq!(file.templates[0].name, "template1");
    }
}

use std::fmt;

/// Represents a segment of a line, potentially containing another block
#[derive(Clone)]
enum LineSegment {
    Content(String),
    Block(Block),
}

impl LineSegment {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineSegment::Content(s) => write!(f, "{}", s),
            LineSegment::Block(b) => b.write_to(f),
        }
    }
}

impl<T: Into<String>> From<T> for LineSegment {
    fn from(v: T) -> Self {
        LineSegment::Content(v.into())
    }
}

/// Represents a single line inside a block of text
#[derive(Clone)]
struct Line(Vec<LineSegment>);

impl<T: Into<String>> From<T> for Line {
    fn from(v: T) -> Self {
        Line(vec![LineSegment::Content(v.into())])
    }
}

impl Line {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.0 {
            segment.write_to(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Block(Vec<Line>);

impl<T: Into<String>> From<T> for Block {
    fn from(v: T) -> Self {
        Block(vec![Line::from(v.into())])
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_to(f)
    }
}

impl Block {
    fn write_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_line = true;
        for line in &self.0 {
            if !first_line {
                write!(f, "\n");
            }
            first_line = false;
            line.write_to(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_a_block() {
        use std::fmt::Write;
        let one_line_block = Block(vec![Line(vec![LineSegment::from("body();")])]);
        let block = Block(vec![
            Line(vec![LineSegment::from("function test() {")]),
            Line(vec![
                LineSegment::from("  "),
                LineSegment::Block(one_line_block.clone()),
            ]),
            Line(vec![LineSegment::from("}")]),
        ]);
        let mut s = String::new();

        // block.write_to(&mut s);
        write!(&mut s, "{}", block);

        assert_eq!(s, "function test() {\n  body();\n}");

        // let result = GrammarParser::parse(Rule::file, TEST_TEMPLATE);
        // let file: File = result.unwrap().next().unwrap().into();
        // assert_eq!(file.templates[0].name, "template1");
    }
}

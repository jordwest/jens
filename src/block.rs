use std::fmt;

/// Represents a segment of a line, potentially containing another block
#[derive(Clone)]
enum LineSegment {
    Content(String),
    Block(Block),
}

impl LineSegment {
    fn write_to(&self, f: &mut fmt::Formatter, prefix: &str) -> fmt::Result {
        match self {
            LineSegment::Content(s) => write!(f, "{}", s),
            LineSegment::Block(b) => b.write_to(f, prefix),
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
    fn write_to(&self, f: &mut fmt::Formatter, prefix: &str) -> fmt::Result {
        let mut sub_prefix = String::from(prefix);
        for segment in &self.0 {
            match segment {
                LineSegment::Content(x) => sub_prefix = sub_prefix + x,
                _ => (),
            }
            segment.write_to(f, &sub_prefix)?;
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
        self.write_to(f, "")
    }
}

impl Block {
    fn write_to(&self, f: &mut fmt::Formatter, prefix: &str) -> fmt::Result {
        let mut first_line = true;
        for line in &self.0 {
            if !first_line {
                write!(f, "\n{}", prefix);
            }
            first_line = false;
            line.write_to(f, prefix)?;
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
        let multi_line_block = Block(vec![
            Line(vec![LineSegment::from("body();")]),
            Line(vec![LineSegment::from("body2();")]),
        ]);

        let make_function_block = |function_body: &Block| {
            Block(vec![
                Line(vec![LineSegment::from("function test() {")]),
                Line(vec![
                    LineSegment::from("  "),
                    LineSegment::Block(function_body.clone()),
                ]),
                Line(vec![LineSegment::from("}")]),
            ])
        };

        let one_func = make_function_block(&one_line_block);
        let mut s1 = String::new();
        write!(&mut s1, "{}", one_func);
        assert_eq!(s1, "function test() {\n  body();\n}");

        let two_funcs = make_function_block(&multi_line_block);
        let mut s2 = String::new();
        write!(&mut s2, "{}", two_funcs);
        assert_eq!(s2, "function test() {\n  body();\n  body2();\n}");
    }
}

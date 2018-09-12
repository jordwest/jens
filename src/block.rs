use std::fmt;

/// Replace every character in a string with a space, but preserve tabs
fn replace_chars_with_whitespace(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    for c in line.chars() {
        match c {
            '\t' => out.push('\t'),
            _ => out.push(' '),
        }
    }
    out
}

/// Represents a segment of a line, potentially containing another block
#[derive(Clone)]
enum LineSegment {
    Content(String),
    Placeholder(String),
    Block(Block),
}

impl LineSegment {
    fn write_to(&self, f: &mut fmt::Formatter, prefix: &str) -> fmt::Result {
        match self {
            LineSegment::Content(s) => write!(f, "{}", s),
            LineSegment::Placeholder(s) => write!(f, "${{{}}}", s),
            LineSegment::Block(b) => {
                let prefix = replace_chars_with_whitespace(prefix);
                b.write_to(f, &prefix)
            }
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
    fn outputs_a_block_with_correct_indentation() {
        use std::fmt::Write;

        let arg_list = Block(vec![
            Line(vec![LineSegment::from("arg1: string,")]),
            Line(vec![LineSegment::from("arg2: number,")]),
            Line(vec![LineSegment::from("arg3: Object")]),
        ]);

        let function_body = Block(vec![
            Line(vec![LineSegment::from("body();")]),
            Line(vec![LineSegment::from("body2();")]),
        ]);

        let function = Block(vec![
            Line(vec![
                LineSegment::from("function test("),
                LineSegment::Block(arg_list),
                LineSegment::from(") {"),
            ]),
            Line(vec![
                LineSegment::from("  "),
                LineSegment::Block(function_body.clone()),
            ]),
            Line(vec![LineSegment::from("}")]),
        ]);

        let mut s = String::new();
        write!(&mut s, "{}", function);
        assert_eq!(
            s,
            include_str!("./snapshots/block.outputs_a_block_with_correct_indentation.txt")
        );
    }
}
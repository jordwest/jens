use crate::parser::{segment::Segment, template::Template};
use std::fmt;

/// When mapping over an iterable, this returns the location of the current iteration
pub enum IteratorLocation {
    /// This item is the first item in the iterable
    First,

    /// This item is not the first or last item, but the nth item
    Nth(usize),

    /// This item is the last in the iterable
    Last,

    /// This item is the only item in the iterable (ie, the first AND last)
    Only,
}

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
#[derive(Clone, PartialEq, Debug)]
pub enum LineSegment {
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

    fn replace(&mut self, new_segment: LineSegment) {
        match self.clone() {
            LineSegment::Placeholder(_) => {
                *self = new_segment;
            }
            _ => (),
        }
    }
}

impl<T: Into<String>> From<T> for LineSegment {
    fn from(v: T) -> Self {
        LineSegment::Content(v.into())
    }
}

/// Represents a single line inside a block of text
#[derive(Clone, PartialEq, Debug)]
pub struct Line(pub Vec<LineSegment>);

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

    pub fn set(&mut self, placeholder_name: &str, content: &Block) {
        for segment in &mut self.0 {
            match segment.clone() {
                LineSegment::Placeholder(ref name) if name == placeholder_name => {
                    segment.replace(LineSegment::Block(content.clone()));
                }
                _ => (),
            }
        }
    }
}

/// A `Block` is one or many lines of text. More blocks can be embedded within a
/// line, in which case the indentation of the previous line will be preserved when
/// outputting new lines.
#[derive(Clone, PartialEq, Debug)]
pub struct Block(pub Vec<Line>);

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
    pub fn empty() -> Self {
        Block(vec![])
    }

    pub fn write_to(&self, f: &mut fmt::Formatter, prefix: &str) -> fmt::Result {
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

    pub fn set<T: Into<Block>>(mut self, placeholder_name: &str, content: T) -> Self {
        let content: &Block = &content.into();
        for line in &mut self.0 {
            line.set(placeholder_name, content);
        }
        self
    }

    /// Run a function that maps over each item in an iterator, then join the results.
    ///
    /// Provides an `IteratorLocation` for checking whether the current item is the
    /// first/last/only/nth item in the list.
    pub fn join_map<T, U, F>(iter: T, mapper: F) -> Self
    where
        T: IntoIterator<Item = U>,
        F: Fn(U, IteratorLocation) -> Block,
    {
        let items: Vec<U> = iter.into_iter().collect();
        let count = items.len();
        match count {
            0 => Block::empty(),
            1 => mapper(items.into_iter().next().unwrap(), IteratorLocation::Only),
            count => Block::join(items.into_iter().enumerate().map(|(i, item)| {
                let loc = match (i, count) {
                    (0, _) => IteratorLocation::First,
                    (x, y) if x == y => IteratorLocation::Last,
                    (x, _) => IteratorLocation::Nth(x),
                };
                mapper(item, loc)
            })),
        }
    }

    /// Repeat a template for each element of some iterable value.
    ///
    /// Deprecated in favor of `join_map`.
    #[deprecated]
    pub fn for_each<T, U, F>(self, iter: T, mapper: F) -> Self
    where
        T: IntoIterator<Item = U>,
        F: Fn(U, Block) -> Block,
    {
        Block::join(iter.into_iter().map(|item| mapper(item, self.clone())))
    }

    /// Join multiple blocks into a single block
    pub fn join<T>(blocks: T) -> Block
    where
        T: IntoIterator<Item = Block>,
    {
        Block(
            blocks
                .into_iter()
                .map(|block| Line(vec![LineSegment::Block(block)]))
                .collect(),
        )
    }
}

impl<'a> From<&'a Template> for Block {
    fn from(t: &'a Template) -> Self {
        let mut lines: Vec<Line> = Vec::with_capacity(t.lines.len());
        let indent_ignored = t.indent_ignored;

        for template_line in &t.lines {
            let mut segments: Vec<LineSegment> =
                Vec::with_capacity(template_line.segments.len() + 1);

            // Add correct amount of whitespace at the beginning of the block
            let indentation_len = template_line.indentation.len();
            if indentation_len > indent_ignored {
                let indentation: &str = &template_line.indentation[indent_ignored..];
                segments.push(LineSegment::Content(String::from(indentation)));
            }

            for template_segment in &template_line.segments {
                segments.push(match template_segment {
                    Segment::Placeholder(x) => LineSegment::Placeholder(x.clone()),
                    Segment::Content(x) => LineSegment::Content(x.clone()),
                })
            }
            lines.push(Line(segments));
        }
        Block(lines)
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

    #[test]
    fn replaces_a_placeholder() {
        let block = Block(vec![Line(vec![
            LineSegment::from("A"),
            LineSegment::Placeholder(String::from("x")),
            LineSegment::from("C"),
        ])]);
        let block = block.set("x", Block(vec![Line(vec![LineSegment::from("B")])]));

        assert_eq!(
            block,
            Block(vec![Line(vec![
                LineSegment::from("A"),
                LineSegment::Block(Block(vec![Line(vec![LineSegment::from("B"),])])),
                LineSegment::from("C"),
            ])])
        );
    }
}

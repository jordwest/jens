use block::{Block, Line, LineSegment};
use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug, PartialEq)]
enum Segment {
    Content(String),
    Placeholder(String),
}

impl<'a> From<Pair<'a, Rule>> for Segment {
    fn from(pair: Pair<'a, Rule>) -> Segment {
        match pair.as_rule() {
            Rule::not_placeholder => Segment::Content(String::from(pair.as_str())),
            Rule::placeholder => {
                Segment::Placeholder(String::from(pair.into_inner().next().unwrap().as_str()))
            }
            unknown => panic!("Unexpected rule '{:?}' found", unknown),
        }
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;

#[derive(Debug)]
struct TemplateLine {
    indentation: String,
    content: Vec<Segment>,
}

impl TemplateLine {
    fn empty() -> Self {
        TemplateLine {
            indentation: String::from(""),
            content: vec![],
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for TemplateLine {
    fn from(pair: Pair<'a, Rule>) -> TemplateLine {
        let mut indentation = String::from("");
        let mut content = vec![];
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::significant_whitespace => indentation = String::from(item.as_str()),
                Rule::template_content => {
                    let result = GrammarParser::parse(Rule::template_phase2, item.as_str())
                        .unwrap()
                        .next()
                        .unwrap();
                    for segment in result.into_inner() {
                        content.push(Segment::from(segment));
                    }
                }
                unknown => panic!("Unexpected rule '{:?}' found", unknown),
            }
        }
        TemplateLine {
            indentation,
            content,
        }
    }
}

#[derive(Debug)]
struct Template {
    name: String,
    indent_ignored: usize,
    lines: Vec<TemplateLine>,
}

impl<'a> From<Pair<'a, Rule>> for Template {
    fn from(pair: Pair<'a, Rule>) -> Template {
        let mut name = String::from("noname");
        let mut indent_ignored = 0;
        let mut lines = vec![];
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::template_decl => {
                    name = String::from(item.into_inner().next().unwrap().as_str())
                }
                Rule::template_line => lines.push(TemplateLine::from(item)),
                Rule::template_terminator => indent_ignored = item.as_str().len() - 1,
                Rule::template_empty_line => lines.push(TemplateLine::empty()),
                unknown => panic!("Unexpected rule '{:?}' found", unknown),
            }
        }
        Template {
            name,
            indent_ignored,
            lines,
        }
    }
}

impl<'a> From<&'a Template> for Block {
    fn from(t: &'a Template) -> Block {
        let mut lines: Vec<Line> = Vec::with_capacity(t.lines.len());
        for template_line in &t.lines {
            let mut segments: Vec<LineSegment> =
                Vec::with_capacity(template_line.content.len() + 1);
            for template_segment in &template_line.content {
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

#[derive(Debug)]
struct File {
    templates: Vec<Template>,
}

impl<'a> From<Pair<'a, Rule>> for File {
    fn from(pair: Pair<'a, Rule>) -> File {
        let mut v: Vec<Template> = vec![];
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::template => v.push(Template::from(item)),
                unknown => panic!("Unexpected rule '{:?}' found", unknown),
            }
        }
        File { templates: v }
    }
}

impl File {
    fn get_template_block(&self, template_name: &str) -> Option<Block> {
        for t in &self.templates {
            if t.name == template_name {
                return Some(t.into());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TEMPLATE: &str = "template1 =
    line 1 with ${placeholder} in the middle
----

template2 =
  a line without a placeholder
  another line
--
";

    #[test]
    fn parses_a_template() {
        let result = GrammarParser::parse(Rule::file, TEST_TEMPLATE);
        let file: File = result.unwrap().next().unwrap().into();
        assert_eq!(file.templates[0].name, "template1");
        assert_eq!(file.templates[0].indent_ignored, 4);
        assert_eq!(
            file.templates[0].lines[0].content[1],
            Segment::Placeholder(String::from("placeholder"))
        );
        assert_eq!(file.templates[1].name, "template2");
        assert_eq!(file.templates[1].indent_ignored, 2);
        assert_eq!(
            file.templates[1].lines[1].content[0],
            Segment::Content(String::from("another line"))
        );

        let template = file
            .get_template_block("template2")
            .expect("template2 should exist");
        println!("-----\n{}\n-----", template);
    }
}

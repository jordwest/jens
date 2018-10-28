use block::{Block, Line, LineSegment};
use crate::parser::{segment::Segment, template_line::TemplateLine, Rule};
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub(crate) struct Template {
    pub(crate) name: String,
    pub(crate) indent_ignored: usize,
    pub(crate) lines: Vec<TemplateLine>,
}

impl Default for Template {
    fn default() -> Self {
        Self {
            name: "no-name".into(),
            indent_ignored: 0,
            lines: vec![],
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for Template {
    fn from(pair: Pair<'a, Rule>) -> Template {
        let mut template = Template::default();

        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::template_decl => {
                    template.name = item.into_inner().next().unwrap().as_str().into()
                }
                Rule::template_line => template.lines.push(TemplateLine::from(item)),
                Rule::template_terminator => template.indent_ignored = item.as_str().len() - 1,
                Rule::template_empty_line => template.lines.push(TemplateLine::default()),
                unknown => panic!("Unexpected rule '{:?}' found", unknown),
            }
        }
        template
    }
}

impl<'a> From<&'a Template> for Block {
    fn from(t: &'a Template) -> Block {
        let mut lines: Vec<Line> = Vec::with_capacity(t.lines.len());
        let indent_ignored = t.indent_ignored;

        for template_line in &t.lines {
            let mut segments: Vec<LineSegment> =
                Vec::with_capacity(template_line.content.len() + 1);

            // Add correct amount of whitespace at the beginning of the block
            let indentation_len = template_line.indentation.len();
            if indentation_len > indent_ignored {
                let indentation: &str = &template_line.indentation[indent_ignored..];
                segments.push(LineSegment::Content(String::from(indentation)));
            }

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

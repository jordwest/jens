use crate::parser::{parse_phase2, segment::Segment, Rule};
use pest::iterators::Pair;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Template {
    pub(crate) name: String,
    pub(crate) indent_ignored: usize,
    pub(crate) lines: Vec<TemplateLine>,
}

impl<'a> From<Pair<'a, Rule>> for Template {
    fn from(pair: Pair<'a, Rule>) -> Template {
        let mut template = Template::default();

        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::template_decl => {
                    template.name = item.into_inner().next().unwrap().as_str().into()
                }
                Rule::template_line => template.lines.push(item.into()),
                Rule::template_terminator => template.indent_ignored = item.as_str().len() - 1,
                Rule::template_empty_line => template.lines.push(TemplateLine::default()),
                _ => unreachable!(),
            }
        }
        template
    }
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TemplateLine {
    pub(crate) indentation: String,
    pub(crate) content: Vec<Segment>,
}

impl<'a> From<Pair<'a, Rule>> for TemplateLine {
    fn from(pair: Pair<'a, Rule>) -> TemplateLine {
        let mut indentation = String::new();
        let mut content = vec![];
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::significant_whitespace => indentation = item.as_str().into(),
                Rule::template_content => content = parse_phase2(item.as_str()).unwrap(),
                _ => unreachable!(),
            }
        }
        TemplateLine {
            indentation,
            content,
        }
    }
}

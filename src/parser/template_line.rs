use crate::parser::{segment::Segment, GrammarParser, Rule};
use pest::{iterators::Pair, Parser};

#[derive(Debug, Default, PartialEq)]
pub(crate) struct TemplateLine {
    pub(crate) indentation: String,
    pub(crate) content: Vec<Segment>,
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

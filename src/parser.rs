use self::template::Template;
use pest::{self, Parser};

pub(crate) mod template;

mod segment;
mod template_line;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest"); // relative to this file

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;

pub(crate) fn parse(content: &str) -> Result<Vec<Template>, pest::Error<Rule>> {
    GrammarParser::parse(Rule::file, content)
        .and_then(|mut pairs| Ok(pairs.next().unwrap()))
        .and_then(|pair| {
            let mut templates: Vec<Template> = vec![];
            for item in pair.into_inner() {
                match item.as_rule() {
                    Rule::template => templates.push(Template::from(item)),
                    unknown => panic!("Unexpected rule '{:?}' found", unknown),
                }
            }
            Ok(templates)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{segment::Segment, template_line::TemplateLine};

    const TEST_TEMPLATE: &str = "template1 =
    line 1 with ${placeholder} in the middle
----

template2 =
  a line without a placeholder
  but with an \\${escaped} dollar sign
--
";

    #[test]
    fn parses_a_template() {
        assert_eq!(
            parse(TEST_TEMPLATE).unwrap(),
            vec![
                Template {
                    name: "template1".into(),
                    indent_ignored: 4,
                    lines: vec![TemplateLine {
                        indentation: "    ".into(),
                        content: vec![
                            Segment::Content("line 1 with ".into()),
                            Segment::Placeholder("placeholder".into()),
                            Segment::Content(" in the middle".into()),
                        ],
                    },]
                },
                Template {
                    name: "template2".into(),
                    indent_ignored: 2,
                    lines: vec![
                        TemplateLine {
                            indentation: "  ".into(),
                            content: vec![Segment::Content("a line without a placeholder".into())],
                        },
                        TemplateLine {
                            indentation: "  ".into(),
                            content: vec![
                                Segment::Content("but with an ".into()),
                                Segment::Content("$".into()),
                                Segment::Content("{escaped} dollar sign".into()),
                            ]
                        }
                    ]
                }
            ]
        );
    }
}

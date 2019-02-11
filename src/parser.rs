use self::{segment::Segment, template::Template};
use pest::{error::Error as PestError, iterators::Pair, Parser};

pub(crate) mod segment;
pub(crate) mod template;

// TODO: Rename to JensParser.
#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct GrammarParser;

pub(crate) fn parse(content: &str) -> Result<Vec<Template>, PestError<Rule>> {
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

// TODO: parser::parse_phase2
// Attempt to make the parsing single phase, otherwise clean up this function.
pub(crate) fn parse_phase2(content: &str) -> Result<Vec<Segment>, PestError<Rule>> {
    GrammarParser::parse(Rule::template_phase2, content)
        .and_then(|mut pairs| Ok(pairs.next().unwrap()))
        .and_then(|pairs| Ok(pairs.into_inner().map(Segment::from).collect()))
}

pub(crate) fn get_ident(pair: Pair<'_, Rule>) -> String {
    pair.into_inner().nth(0).unwrap().as_str().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{segment::Segment, template::TemplateLine};

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
                        segments: vec![
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
                            segments: vec![Segment::Content("a line without a placeholder".into())],
                        },
                        TemplateLine {
                            indentation: "  ".into(),
                            segments: vec![
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

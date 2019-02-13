use self::{segment::Segment, template::Template};
use pest::{error::Error as PestError, iterators::Pair, Parser};
use pest_derive::Parser;

pub(crate) mod segment;
pub(crate) mod template;

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
                    Rule::EOI => {}
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
        .and_then(|pairs| {
            Ok(pairs
                .into_inner()
                .filter_map(|pair| {
                    if Rule::EOI == pair.as_rule() {
                        return None;
                    }
                    Some(Segment::from(pair))
                })
                .collect())
        })
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
        use insta::assert_debug_snapshot_matches;
        assert_debug_snapshot_matches!("parser.parses_a_template", parse(TEST_TEMPLATE).unwrap());
    }
}

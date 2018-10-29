use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub(crate) enum Segment {
    Content(String),
    Placeholder(String),
}

impl<'a> From<Pair<'a, Rule>> for Segment {
    fn from(pair: Pair<'a, Rule>) -> Segment {
        match pair.as_rule() {
            Rule::escaped_dollar => Segment::Content(String::from("$")),
            Rule::not_placeholder => Segment::Content(String::from(pair.as_str())),
            Rule::placeholder => {
                Segment::Placeholder(String::from(pair.into_inner().next().unwrap().as_str()))
            }
            unknown => panic!("Unexpected rule '{:?}' found", unknown),
        }
    }
}

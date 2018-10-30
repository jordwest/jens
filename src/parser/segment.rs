use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub(crate) enum Segment {
    Content(String),
    Placeholder(String),
}

impl From<Pair<'_, Rule>> for Segment {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::escaped_dollar => Segment::Content("$".into()),
            Rule::not_placeholder => Segment::Content(pair.as_str().into()),
            Rule::placeholder => Segment::Placeholder(get_ident(pair)),
            _ => unreachable!(),
        }
    }
}

fn get_ident(pair: Pair<'_, Rule>) -> String {
    pair.into_inner().nth(0).unwrap().as_str().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn tmpl_line(content: &str) -> String {
        format!("main =\n    {}\n----", content)
    }

    #[test]
    fn placeholder() {
        let templates = parse(&tmpl_line("${x}")).unwrap();
        let segments = &templates[0].lines[0].content;

        assert_eq!(segments, &[Segment::Placeholder("x".into())]);
    }

    #[test]
    fn content() {
        let templates = parse(&tmpl_line("content")).unwrap();
        let segments = &templates[0].lines[0].content;

        assert_eq!(segments, &[Segment::Content("content".into())]);
    }

    #[test]
    fn escaped_dollar() {
        let templates = parse(&tmpl_line("\\$")).unwrap();
        let segments = &templates[0].lines[0].content;

        assert_eq!(segments, &[Segment::Content("$".into())]);
    }

    #[test]
    fn escaped_placeholder() {
        let templates = parse(&tmpl_line("\\${x}")).unwrap();
        let segments = &templates[0].lines[0].content;

        assert_eq!(
            segments,
            &[Segment::Content("$".into()), Segment::Content("{x}".into())]
        );
    }
}

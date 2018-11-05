use crate::parser::{get_ident, parse_phase2, segment::Segment, Rule};
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
                Rule::template_decl => template.name = get_ident(item),
                Rule::template_line => template.lines.push(item.into()),
                Rule::template_terminator => {
                    template.indent_ignored = item.as_str().matches("-").count()
                }
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
    pub(crate) segments: Vec<Segment>,
}

impl<'a> From<Pair<'a, Rule>> for TemplateLine {
    fn from(pair: Pair<'a, Rule>) -> TemplateLine {
        let mut indentation = String::new();
        let mut segments = vec![];
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::significant_whitespace => indentation = item.as_str().into(),
                Rule::template_content => segments = parse_phase2(item.as_str()).unwrap(),
                _ => unreachable!(),
            }
        }
        TemplateLine {
            indentation,
            segments,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse, segment::Segment};

    #[test]
    fn empty_template() {
        let templates = parse("main =\n----").unwrap();

        assert_eq!(
            templates,
            &[Template {
                name: "main".into(),
                indent_ignored: 4,
                lines: vec![]
            }]
        );
    }

    #[test]
    fn space_indented_content() {
        let templates = parse("main =\n    indent4\n     indent5\n----").unwrap();

        assert_eq!(
            templates,
            &[Template {
                name: "main".into(),
                indent_ignored: 4,
                lines: vec![
                    TemplateLine {
                        indentation: "    ".into(),
                        segments: vec![Segment::Content("indent4".into())],
                    },
                    TemplateLine {
                        indentation: "     ".into(),
                        segments: vec![Segment::Content("indent5".into())],
                    }
                ]
            }]
        );
    }

    #[test]
    fn tab_indented() {
        let templates = parse("main =\n\tindent1\n\t\tindent2\n-").unwrap();

        assert_eq!(
            templates,
            &[Template {
                name: "main".into(),
                indent_ignored: 1,
                lines: vec![
                    TemplateLine {
                        indentation: "\t".into(),
                        segments: vec![Segment::Content("indent1".into())],
                    },
                    TemplateLine {
                        indentation: "\t\t".into(),
                        segments: vec![Segment::Content("indent2".into())],
                    }
                ]
            }]
        );
    }

    #[test]
    fn mixed_indentation() {
        let templates = parse("main =\n    \tindent\n----").unwrap();

        assert_eq!(
            templates,
            &[Template {
                name: "main".into(),
                indent_ignored: 4,
                lines: vec![TemplateLine {
                    indentation: "    \t".into(),
                    segments: vec![Segment::Content("indent".into())],
                }]
            }]
        );
    }

    #[test]
    fn ignore_more_indentation() {
        let templates = parse("main =\n  content\n----").unwrap();

        assert_eq!(
            templates,
            &[Template {
                name: "main".into(),
                indent_ignored: 4,
                lines: vec![TemplateLine {
                    indentation: "  ".into(),
                    segments: vec![Segment::Content("content".into())],
                }]
            }]
        );
    }

    #[test]
    fn ignore_inner_template() {
        let templates = parse("main =\n    main =\n        x\n    ----\n----").unwrap();

        assert_eq!(
            templates,
            &[Template {
                name: "main".into(),
                indent_ignored: 4,
                lines: vec![
                    TemplateLine {
                        indentation: "    ".into(),
                        segments: vec![Segment::Content("main =".into())],
                    },
                    TemplateLine {
                        indentation: "        ".into(),
                        segments: vec![Segment::Content("x".into())],
                    },
                    TemplateLine {
                        indentation: "    ".into(),
                        segments: vec![Segment::Content("----".into())],
                    }
                ]
            }]
        );
    }

    #[test]
    fn invalid_start() {
        assert!(parse("main\n  content\n--").is_err())
    }

    #[test]
    fn invalid_end() {
        assert!(parse("main =\n content\n").is_err())
    }
}

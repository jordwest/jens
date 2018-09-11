extern crate pest;
#[macro_use]
extern crate pest_derive;

mod block;
mod grammar;

#[derive(Debug, Clone)]
pub enum TemplatePart {
    Text(String),
    Subtemplate(String, Template),
}

#[derive(Debug, Clone)]
pub struct Template {
    parts: Vec<TemplatePart>,
}

#[derive(Clone)]
struct ParseState {
    directive: String,
}

#[derive(Debug)]
enum BlockMarker {
    Begin,
    End,
}

impl BlockMarker {
    fn parse(arg: &str) -> Option<BlockMarker> {
        match arg {
            "begin" => Some(BlockMarker::Begin),
            "end" => Some(BlockMarker::End),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Command {
    Comment { marker: BlockMarker },
    LineTemplate { name: String },
    SetDirective { directive: String },
    Template { marker: BlockMarker, name: String },
}

impl Command {
    fn parse(parameters: Vec<&str>) -> Command {
        let command_name = parameters.get(0).expect("No command specified");
        match command_name {
            &"set-directive" => {
                let directive = parameters.get(1).expect("No directive specified");
                Command::SetDirective {
                    directive: String::from(*directive),
                }
            }
            &"line-template" => {
                let template_name = parameters
                    .get(1)
                    .expect("Must specify template name as second arg of line-template command");

                Command::LineTemplate {
                    name: String::from(*template_name),
                }
            }
            &"template" => {
                let block_marker = parameters
                    .get(1)
                    .expect("Must specify begin/end as second arg");
                let template_name = parameters
                    .get(2)
                    .expect("Must specify template name as third arg of template command");

                Command::Template {
                    name: String::from(*template_name),
                    marker: BlockMarker::parse(block_marker)
                        .expect("Must specify begin/end as second arg"),
                }
            }
            &"comment" => {
                let block_marker = parameters
                    .get(1)
                    .expect("Must specify begin/end as second arg");

                Command::Comment {
                    marker: BlockMarker::parse(block_marker)
                        .expect("Must specify begin/end as second arg"),
                }
            }
            other => panic!(format!("Unknown command '{}'", other)),
        }
    }
}

enum LineReaderState<'a> {
    /// Currently reading lines of the template
    Normal,

    /// Currently reading a comment
    Comment,

    /// Currently reading lines of a subtemplate
    Subtemplate(String, Vec<&'a str>),

    /// The next line will be a template on its own
    LineTemplate(String),
}

impl Template {
    /// Replaces a matching string anywhere inside this template and subtemplates
    pub fn replace(&mut self, find: &str, replace: &str) {
        for mut part in &mut self.parts {
            match part {
                TemplatePart::Text(s) => *s = s.replace(find, replace),
                TemplatePart::Subtemplate(_, t) => t.replace(find, replace),
            }
        }
    }

    /// Process a specific template section
    pub fn with_template(&mut self, section_name: &str, map: &Fn(&mut Template)) {
        for part in &mut self.parts {
            match part {
                TemplatePart::Subtemplate(ref k, t) if k == section_name => {
                    map(t);
                }
                _ => (),
            }
        }
    }

    /// Repeats a template section for each of `values`, running `map` over each instance
    pub fn repeat_template<T>(
        &mut self,
        section_name: &str,
        values: &Vec<T>,
        map: &Fn(&T, &mut Template),
    ) {
        let mut new_parts = vec![];
        for part in &mut self.parts {
            match part {
                TemplatePart::Subtemplate(ref k, ref t) if k == section_name => {
                    for val in values {
                        let mut template: Template = t.clone();
                        map(val, &mut template);
                        new_parts.push(TemplatePart::Subtemplate(k.clone(), template));
                    }
                }
                part => new_parts.push(part.clone()),
            }
        }
        self.parts = new_parts;
    }

    // /// Keeps only the section with the given name, discarding all other template sections
    // pub fn keep_only(&mut self, section_name: &str)

    pub fn output(&self) -> String {
        let mut lines = vec![];
        for part in &self.parts {
            match part {
                TemplatePart::Text(ref s) => lines.push(s.clone()),
                TemplatePart::Subtemplate(_, t) => lines.push(t.output()),
            }
        }
        lines.join("\n")
    }

    fn get_directive(line: &str, parse_state: &ParseState) -> Option<Command> {
        if let Some(pos) = line.find(&parse_state.directive) {
            let directive = {
                let mut start = String::from(line);
                start.split_off(pos + parse_state.directive.len())
            };

            let parameters: Vec<&str> = directive.split(' ').collect();
            let command = Command::parse(parameters);
            Some(command)
        } else {
            None
        }
    }

    fn parse_lines(template_str: &Vec<&str>, mut parse_state: ParseState) -> Template {
        let mut state = LineReaderState::Normal;
        let mut template = Template { parts: vec![] };

        for line in template_str {
            let command = Template::get_directive(line, &parse_state);
            let mut next_state = None;
            match state {
                LineReaderState::Subtemplate(ref name, ref mut lines) => match command {
                    Some(Command::Template {
                        marker: BlockMarker::End,
                        name: ref v,
                    })
                        if v == name =>
                    {
                        next_state = Some(LineReaderState::Normal);
                        template.parts.push(TemplatePart::Subtemplate(
                            v.clone(),
                            Template::parse_lines(&lines, parse_state.clone()),
                        ));
                    }
                    _ => lines.push(line),
                },
                LineReaderState::Normal => match command {
                    Some(Command::SetDirective { directive: s }) => {
                        parse_state.directive = s;
                    }
                    Some(Command::LineTemplate { ref name }) => {
                        next_state = Some(LineReaderState::LineTemplate(name.clone()))
                    }
                    Some(Command::Template {
                        marker: BlockMarker::Begin,
                        ref name,
                    }) => next_state = Some(LineReaderState::Subtemplate(name.clone(), vec![])),
                    Some(Command::Comment {
                        marker: BlockMarker::Begin,
                    }) => next_state = Some(LineReaderState::Comment),
                    _ => template.parts.push(TemplatePart::Text(String::from(*line))),
                },
                LineReaderState::Comment => match command {
                    Some(Command::Comment {
                        marker: BlockMarker::End,
                    }) => next_state = Some(LineReaderState::Normal),
                    _ => (),
                },
                LineReaderState::LineTemplate(ref name) => {
                    template.parts.push(TemplatePart::Subtemplate(
                        name.clone(),
                        Template {
                            parts: vec![TemplatePart::Text(String::from(*line))],
                        },
                    ));
                    next_state = Some(LineReaderState::Normal);
                }
            }
            if let Some(new_state) = next_state {
                state = new_state;
            }
        }
        match state {
            LineReaderState::Subtemplate(name, _) => {
                panic!(format!("Subtemplate <{}> was never closed", name))
            }
            _ => (),
        }
        template
    }

    /// Parse template source into a Template
    pub fn parse(template_str: &str) -> Template {
        let parse_state = ParseState {
            directive: String::from("#"),
        };
        Template::parse_lines(&template_str.split('\n').into_iter().collect(), parse_state)
    }
}

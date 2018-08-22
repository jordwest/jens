extern crate regex;

use regex::Regex;

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

    fn parse_lines(template_str: &Vec<&str>, parse_state: ParseState) -> Template {
        let opening_tag = Regex::new(r"<([A-z]+)>").unwrap();
        let closing_tag = Regex::new(r"</([A-z]+)>").unwrap();
        let single_tag = Regex::new(r"\^\^ ([A-z]+)").unwrap();

        let mut current_section: Option<String> = None;
        let mut section_lines: Vec<&str> = vec![];
        let mut template = Template { parts: vec![] };
        for line in template_str {
            if let Some(v) = current_section.clone() {
                if closing_tag.is_match(line) {
                    let captures = closing_tag.captures(line).unwrap();
                    let section_name = captures.get(1).unwrap().as_str();
                    if section_name == v {
                        current_section = None;
                        template.parts.push(TemplatePart::Subtemplate(
                            String::from(section_name),
                            Template::parse_lines(&section_lines, parse_state.clone()),
                        ));
                    } else {
                        section_lines.push(line);
                    }
                } else {
                    section_lines.push(line);
                }
            } else if opening_tag.is_match(line) {
                let captures = opening_tag.captures(line).unwrap();
                let section_name = captures.get(1).unwrap().as_str();
                current_section = Some(String::from(section_name));
                section_lines = vec![];
            } else if single_tag.is_match(line) {
                let captures = single_tag.captures(line).unwrap();
                let section_name = captures.get(1).unwrap().as_str();
                let last_line = template.parts.pop().unwrap();
                template.parts.push(TemplatePart::Subtemplate(
                    String::from(section_name),
                    Template {
                        parts: vec![last_line],
                    },
                ))
            } else {
                template.parts.push(TemplatePart::Text(String::from(*line)));
            }
        }
        if let Some(v) = current_section {
            panic!(format!("Template section <{}> was never closed", v));
        }
        template
    }

    pub fn parse(template_str: &str) -> Template {
        let parse_state = ParseState {
            directive: String::from("#"),
        };
        Template::parse_lines(&template_str.split('\n').into_iter().collect(), parse_state)
    }
}

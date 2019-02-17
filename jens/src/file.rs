use crate::{
    block::Block,
    parser::{self, template::Template},
};

#[derive(Debug)]
pub struct File {
    pub templates: Vec<Template>,
}

impl File {
    // TODO: Add custom parse error
    pub fn parse(content: &str) -> Result<Self, ()> {
        parser::parse(content)
            .map(|templates| File { templates })
            .map_err(|_| ())
    }

    /// Find a template in the template definition file.
    pub fn template_opt(&self, template_name: &str) -> Option<Block> {
        for t in &self.templates {
            if t.name == template_name {
                return Some(t.into());
            }
        }
        None
    }

    /// Find a template in the template definition file. Panics if not found.
    pub fn template(&self, template_name: &str) -> Block {
        self.template_opt(template_name).unwrap()
    }
}

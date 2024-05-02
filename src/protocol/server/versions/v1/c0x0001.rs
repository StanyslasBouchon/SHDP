use std::{fs::File, io::Read, path::Path, str::from_utf8};

use ego_tree::NodeRef;
use html_minifier::HTMLMinifier;
use scraper::{Html, Node};

use crate::protocol::{
    errors::{Error, ErrorKind},
    server::{bits::builder::InBuilder, event::EventBuilder},
};

pub struct HtmlFileResponse {
    builder: InBuilder,
    path: String,
}

impl HtmlFileResponse {
    pub fn new(path: String) -> Self {
        if cfg!(feature = "debug") {
            println!(
                "[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;205m0x0001\x1b[0m created ({})",
                path
            );
        }

        HtmlFileResponse {
            builder: InBuilder::new(),
            path,
        }
    }

    fn append_text(&mut self, node: NodeRef<'_, Node>, text: &String) -> Result<(), Error> {
        if text.trim().is_empty() {
            return Ok(());
        }

        if node.parent().is_none() {
            return Ok(());
        }

        let parent = node.parent().unwrap();

        match parent.value().as_element() {
            Some(element) => {
                if element.name() == "pre" {
                    return Ok(());
                }
            }
            None => return Ok(()),
        }

        self.builder.add_data(0, 10)?;
        self.builder.add_data(text.len() as u32, 15)?;
        self.builder.add_bytes(text.as_bytes())?;

        Ok(())
    }

    fn process_node(
        &mut self,
        node: NodeRef<'_, Node>,
        open_elements: &mut Vec<String>,
    ) -> Result<(), Error> {
        match node.value() {
            Node::Element(element) => {
                let element_name = &element.name().to_string();

                if element_name == "html" {
                    for child in node.children() {
                        self.process_node(child, open_elements)?;
                    }
                }

                open_elements.push(element_name.clone().to_owned());

                self.builder.add_data(16, 10)?;
                self.builder.add_bytes(element_name.as_bytes())?;

                if element.attrs.len() > 0 {
                    self.builder.add_data(17, 10)?;

                    for (name, value) in element.attrs() {
                        self.builder.add_bytes(name.as_bytes())?;
                        self.append_text(node, &value.to_string())?;
                    }
                }

                self.builder.add_data(24, 10)?;

                for child in node.children() {
                    self.process_node(child, open_elements)?;
                }

                open_elements.pop();

                self.builder.add_data(25, 10)?;
            }
            Node::Text(text) => {
                self.append_text(node, &text.to_string())?;
            }
            _ => (),
        }

        Ok(())
    }
}

impl EventBuilder for HtmlFileResponse {
    fn construct(&mut self) -> Result<(), Error> {
        let html_file_name = Path::new(self.path.as_str())
            .file_name()
            .ok_or(Error {
                code: 400,
                message: format!("Invalid file name: {}", self.path),
                kind: ErrorKind::BadRequest,
            })?
            .to_str()
            .ok_or(Error {
                code: 400,
                message: format!("Invalid file name: {}", self.path),
                kind: ErrorKind::BadRequest,
            })?
            .to_string();

        self.builder.add_bytes(html_file_name.as_bytes())?;
        self.builder.add_data(0, 8)?;

        let minified_html = get_minified_html_file(self.path.clone())?;
        let document = Html::parse_fragment(&minified_html);

        let mut open_elements = Vec::<String>::new();
        for node in document.tree.root().children() {
            self.process_node(node, &mut open_elements)?;
        }

        Ok(())
    }

    fn get_builder(&self) -> &InBuilder {
        &self.builder
    }

    fn get_event(&self) -> u16 {
        0x0001
    }
}

fn get_minified_html_file(path: String) -> Result<String, Error> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            return Err(Error {
                code: 404,
                message: format!("File not found: {}", path),
                kind: ErrorKind::NotFound,
            });
        }
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error {
                code: 500,
                message: format!("File read error: ({}): {}", path, e.to_string()),
                kind: ErrorKind::InternalServerError,
            });
        }
    };

    let mut minifier = HTMLMinifier::new();
    minifier.set_minify_code(true);
    minifier.set_remove_comments(true);
    match minifier.digest(content) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error {
                code: 500,
                message: format!("HTML minify error: ({}): {}", path, e.to_string()),
                kind: ErrorKind::InternalServerError,
            });
        }
    }

    let result = match from_utf8(minifier.get_html()) {
        Ok(result) => result.to_string(),
        Err(e) => {
            return Err(Error {
                code: 500,
                message: format!("HTML minify error: ({}): {}", path, e.to_string()),
                kind: ErrorKind::InternalServerError,
            })
        }
    };

    Ok(result)
}

use std::{ fs::File, io::Read, path::Path, str::from_utf8 };

use bitvec::order::Lsb0;
use ego_tree::NodeRef;
use html_minifier::HTMLMinifier;
use scraper::{ Html, Node };

use crate::protocol::{
    errors::{ Error, ErrorKind },
    managers::{ bits::encoder::BitEncoder, event::EventEncoder },
    server::bits::utils::CHARS,
};

pub struct HtmlFileResponse {
    encoder: BitEncoder<Lsb0>,
    path: String,
}

impl HtmlFileResponse {
    pub fn new(path: String) -> Self {
        if cfg!(feature = "debug") {
            println!("[\x1b[38;5;227mSHDP\x1b[0m] \x1b[38;5;205m0x0001\x1b[0m created ({})", path);
        }

        HtmlFileResponse {
            encoder: BitEncoder::<Lsb0>::new(),
            path,
        }
    }

    fn append_text(
        &mut self,
        node: NodeRef<'_, Node>,
        text: &String
    ) -> Result<(), Error> {
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
            None => {
                return Ok(());
            }
        }

        self.encoder.add_data(0, 10)?;
        self.encoder.add_data(text.len() as u32, 15)?;
        self.encoder.add_bytes(text.as_bytes())?;

        Ok(())
    }

    fn append_fyve_text(&mut self, text: &String) -> Result<(), Error> {
        if text.trim().is_empty() {
            return Err(Error {
                code: 400,
                message: "Empty text".to_string(),
                kind: ErrorKind::BadRequest,
            });
        }

        let chars = text.chars().collect::<Vec<char>>();

        for c in chars {
            self.encoder.add_bitvec(CHARS.get(&c).unwrap())?;
        }

        Ok(())
    }

    fn process_node(
        &mut self,
        node: NodeRef<'_, Node>,
        open_elements: &mut Vec<String>
    ) -> Result<(), Error> {
        match node.value() {
            Node::Element(element) => {
                let element_name = &element.name().to_string();

                open_elements.push(element_name.clone().to_owned());

                self.encoder.add_data(16, 10)?;
                self.append_fyve_text(element_name)?;

                if element.attrs.len() > 0 {
                    self.encoder.add_data(17, 10)?;

                    for (name, value) in element.attrs() {
                        self.append_fyve_text(&name.to_string())?;
                        self.append_text(node, &value.to_string())?;
                    }
                }

                self.encoder.add_data(24, 10)?;

                for child in node.children() {
                    self.process_node(child, open_elements)?;
                }

                open_elements.pop();

                self.encoder.add_data(25, 10)?;
            }
            Node::Text(text) => {
                self.append_text(node, &text.to_string())?;
            }
            _ => (),
        }

        Ok(())
    }
}

impl EventEncoder<Lsb0> for HtmlFileResponse {
    fn encode(&mut self) -> Result<(), Error> {
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

        self.encoder.add_bytes(html_file_name.as_bytes())?;
        self.encoder.add_data(0, 8)?;

        let minified_html = get_minified_html_file(self.path.clone())?;
        let document = Html::parse_fragment(&minified_html);

        let mut open_elements = Vec::<String>::new();
        for node in document.tree.root().children() {
            self.process_node(node, &mut open_elements)?;
        }

        Ok(())
    }

    fn get_encoder(&self) -> &BitEncoder<Lsb0> {
        &self.encoder
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
                message: format!(
                    "File read error: ({}): {}",
                    path,
                    e.to_string()
                ),
                kind: ErrorKind::InternalServerError,
            });
        }
    }

    let mut minifier = HTMLMinifier::new();
    minifier.set_minify_code(true);
    minifier.set_remove_comments(true);
    match minifier.digest(content) {
        Ok(_) => (),
        Err(e) => {
            return Err(Error {
                code: 500,
                message: format!(
                    "HTML minify error: ({}): {}",
                    path,
                    e.to_string()
                ),
                kind: ErrorKind::InternalServerError,
            });
        }
    }

    let result = match from_utf8(minifier.get_html()) {
        Ok(result) => result.to_string(),
        Err(e) => {
            return Err(Error {
                code: 500,
                message: format!(
                    "HTML minify error: ({}): {}",
                    path,
                    e.to_string()
                ),
                kind: ErrorKind::InternalServerError,
            });
        }
    };

    Ok(result)
}

use std::fmt;
use std::fs::File;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use raw_code::{CodeItem, CodeConfig, print_code_item};
use string_gen::keywords::*;
use string_gen::{code_to_str};

#[derive(PartialEq, Eq, Hash)]
pub enum Lang {
    Cpp,
    Rust
}

pub struct FormattedCode {
    pub language: Lang,
    pub elements: Vec<CodeItem>,
    pub num_tabs: u8,
    pub tab_char: char,
}

impl FormattedCode {
    pub fn new(lang: Lang, cfg: &Option<&CodeConfig>, data: &Vec<CodeItem>) -> FormattedCode {
        let mut fmt_code = FormattedCode {
            language: lang,
            elements: data.to_vec(),
            num_tabs: 4,
            tab_char: ' '
        };

        match *cfg {
            Some(config) => {
                // replace types and names using data from config file
                fmt_code.process_config(&config);
            },
            None => {}
        };

        fmt_code
    }

    fn process_config(&mut self, config: &CodeConfig) {
        for (k, v) in config.type_dict.iter() {
            for e in self.elements.iter_mut() {
                FormattedCode::update_element(e, &k, &v);
            }
        }
        for (k, v) in config.name_dict.iter() {
            for e in self.elements.iter_mut() {
                FormattedCode::update_element(e, &k, &v);
            }
        }
        for (k, v) in config.global_cfg.iter() {
            match k.as_str() {
                NUM_TABS => self.num_tabs = v.parse::<u8>().unwrap(),
                TAB_CHAR => self.tab_char = v.chars().next().unwrap(),
                _ => {}
            }
        }
    }

    fn update_element(e: &mut CodeItem, key: &String, value: &String) {
        for child in e.children.iter_mut() {
            FormattedCode::update_element(child, key, value);
        }
        for a in e.attributes.iter_mut() {
            if a.1 == *key {
                a.1 = value.clone();
            }
        }
    }

    // save generated code (formatted as string) to file
    pub fn to_file(&self, filename: &str) -> io::Result<()> {
        let code = self.to_string();
        let path = Path::new(&filename);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't open {} for writing: {}", path.display(), why.description()),
            Ok(file) => file,
        };

        file.write_all(code.as_bytes())
    }

    // generate a string with output code
    pub fn to_string(&self) -> String {
        code_to_str(self)
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_name = match *self {
            Lang::Cpp => "C/C++", Lang::Rust => "Rust"
        };
        write!(f, "{}", str_name)
    }
}

impl fmt::Display for FormattedCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Target: {}\n", self.language);
        let _ = write!(f, "*\n");
        for e in self.elements.iter() {
            let mut empty_spaces = Vec::<u8>::new();
            print_code_item(e, f, 0, &mut empty_spaces);
        }
        write!(f, "*\n")
    }
}

use crate::error_lib::ParserError;
use crate::parsers::variant_playlist::VariantPlaylist;
use crate::tag_lib::TagType;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MasterPlaylist {
    master_file: Option<PathBuf>,
    lines: Vec<String>,
    version: Option<u32>,
    independent_segments: bool,
    variants: Vec<VariantPlaylist>,
}

impl MasterPlaylist {
    pub fn new() -> MasterPlaylist {
        MasterPlaylist {
            master_file: None,
            lines: Vec::new(),
            version: None,
            independent_segments: false,
            variants: vec![],
        }
    }

    pub fn parse_from_file(&mut self, master_file: &PathBuf) -> Result<(), ParserError> {
        self.master_file = Some(master_file.clone());
        let file_result = File::open(&master_file);

        if let Ok(file) = file_result {
            let reader = BufReader::new(file);
            let file_lines = reader.lines();

            for (line_number, line_result) in file_lines.enumerate() {
                let line = line_result.unwrap();
                if &line_number == &0 {
                    if line != "#EXTM3U" {
                        return Err(ParserError::InvalidFile);
                    }
                }

                if line.contains("#EXT") {
                    self.parse_tag_line(&line_number, line)
                } else {
                    println!("got data line")
                }
            }
        }

        Ok(())
    }

    fn parse_tag_line(&mut self, line_number: &usize, line_str: String) {
        if line_str.contains(":") {
            let split_str = line_str.split_once(":");
            let Some((tag, tag_data)) = split_str else {
                panic!("tag split failed");
            };

            let tag_type = TagType::new(tag);

            match tag_type {
                TagType::Version(_) => {
                    let parsed = tag_data.parse::<u32>();

                    if let Ok(version) = parsed {
                        self.version = Some(version);
                    }
                }
                TagType::FileType(_) => {}
                TagType::IndependentSegments(_) => {
                    self.independent_segments = true;
                }
                TagType::AlternativeMedia(_) => {}
                TagType::VariantPlaylist(_) => {
                    let variant = VariantPlaylist::parse_from_tag_data(tag_data);
                    println!("{:?}", variant);
                    self.variants.push(variant);
                }
                TagType::Comment(_) => {}
            }

            println!("{} {}\n{}", line_number, tag, tag_data);
        }
    }
}

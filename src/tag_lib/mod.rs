use std::path::PathBuf;

// struct TagLocation {
//     file: PathBuf,
//     line_number: u32,
// }

// struct Tag {
//     pub tag_type: TagType,
//     pub data: Option<String>,
//     pub location: TagLocation,
//     pub description: String,
// }

// impl Tag {
//     pub fn new() -> Tag {}

//     // pub fn as_str(&self) -> String {
//     //     match self.tag_type {
//     //         TagType::FileType(val) => val.to_string(),
//     //         TagType::Version(val) => val.to_string(),
//     //         TagType::IndependentSegments(val) => val.to_string(),
//     //     }
//     // }
// }

pub enum TagType {
    FileType(&'static str),
    Version(&'static str),
    IndependentSegments(&'static str),
    AlternativeMedia(&'static str),
    VariantPlaylist(&'static str),
    Comment(&'static str),
}

impl TagType {
    pub fn new(tag: &str) -> TagType {
        match tag {
            "#EXTM3U" => TagType::FileType("EXTM3U"),
            "#EXT-X-VERSION" => TagType::Version("EXT-X-VERSION"),
            "#EXT-X-INDEPENDENT-SEGMENTS" => {
                TagType::IndependentSegments("EXT-X-INDEPENDENT-SEGMENTS")
            }
            "#EXT-X-MEDIA" => TagType::AlternativeMedia("EXT-X-MEDIA"),
            "#EXT-X-STREAM-INF" => TagType::VariantPlaylist("EXT-X-STREAM-INF"),
            _ => TagType::Comment("#"),
        }
    }
}

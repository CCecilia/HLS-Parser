use std::path::PathBuf;

pub struct MediaPlaylist {
    temp_file_path: PathBuf,
}

impl MediaPlaylist {
    pub fn new(temp_file_path: PathBuf) -> MediaPlaylist {
        MediaPlaylist {
            temp_file_path: temp_file_path,
        }
    }

    pub fn parse() {}
}

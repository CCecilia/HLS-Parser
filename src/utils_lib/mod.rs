use crate::error_lib::PlaylistDownloadError;
use reqwest;
use std::path::PathBuf;
use std::{env, fs, io};

pub async fn make_request_and_save_to_file(
    stream_url: &str,
) -> Result<PathBuf, PlaylistDownloadError> {
    let temp_dir = env::temp_dir();
    let temp_playlist_file_path = temp_dir.join("downloaded_playlist.m3u8");
    let temp_file_result = fs::File::create(&temp_playlist_file_path);

    let Ok(response) = reqwest::get(stream_url).await else {
        return Err(PlaylistDownloadError::RequestFailed);
    };

    let Ok(response_data) = response.text().await else {
        return Err(PlaylistDownloadError::FailedToReadResponse);
    };

    let Ok(mut temp_file) = temp_file_result else {
        return Err(PlaylistDownloadError::FailedToCreateTempFile);
    };

    if io::copy(&mut response_data.as_bytes(), &mut temp_file).is_err() {
        return Err(PlaylistDownloadError::FailedToCopyData);
    }

    Ok(temp_playlist_file_path)
}

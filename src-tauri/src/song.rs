use std::{env, os};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{time::Duration, fs::File, path::Path, io::BufReader};
use anyhow::{bail, Result};
use id3::frame::Lyrics;
use lofty::TaggedFileExt;
use lofty::id3::v2::{Frame, FrameFlags, FrameValue, ID3v2Tag, LanguageFrame};
use lofty::{
    mpeg::MPEGFile, Accessor, AudioFile, FileType, ItemKey, ItemValue, Picture, PictureType,
    TagExt, TagItem, TextEncoding,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Song {
    artist: Option<String>,
    album: Option<String>,
    title: Option<String>,
    name: Option<String>,
    genre: Option<String>,
    
    // bit_rate: Option<i32>,
    // sample_rate: Option<i32>,
    // bit_depth: Option<i8>,

    ext: Option<String>,
    file_type: Option<String>,
    file_path: Option<String>,
    wasm_file_path: Option<String>,
    directory: Option<String>,
    duration: Duration
}

impl Song {
    pub fn read_from_path<P: AsRef<Path>>(path: P, for_db: bool) -> Result<Self> {
        let path = path.as_ref();

        let probe = lofty::Probe::open(path)?;
        let file_type = probe.file_type();

        let mut song = Self::new(path);
        if let Ok(mut tagged_file) = probe.read() {
            // We can at most get the duration and file type at this point
            let properties = tagged_file.properties();
            song.duration = properties.duration();

            if let Some(tag) = tagged_file.primary_tag_mut() {
                // Check for a length tag (Ex. TLEN in ID3v2)
                if let Some(len_tag) = tag.get_string(&ItemKey::Length) {
                    song.duration = Duration::from_millis(len_tag.parse::<u64>()?);
                }

                song.artist = tag.artist().map(|tag| tag.to_string());
                song.album = tag.album().map(|tag| tag.to_string());
                song.title = tag.title().map(|tag| tag.to_string());
                song.genre = tag.genre().map(|tag| tag.to_string());

                if for_db {
                    return Ok(song);
                }

                // Get all of the lyrics tags
                // let mut lyric_frames: Vec<Lyrics> = Vec::new();
                // match file_type {
                //     Some(FileType::MPEG) => {
                //         let mut reader = BufReader::new(File::open(path)?);
                //         // let file = MPEGFile::read_from(&mut reader, false)?;
                //         let file = MPEGFile::read_from(&mut reader, lofty::ParseOptions::new())?;

                //         if let Some(id3v2_tag) = file.id3v2() {
                //             for lyrics_frame in id3v2_tag.unsync_text() {
                //                 let mut language =
                //                     String::from_utf8_lossy(&lyrics_frame.language).to_string();
                //                 if language.len() < 3 {
                //                     language = "eng".to_string();
                //                 }
                //                 lyric_frames.push(Lyrics {
                //                     lang: language,
                //                     description: lyrics_frame.description.clone(),
                //                     text: lyrics_frame.content.clone(),
                //                 });
                //             }
                //         }
                //     }
                //     _ => {
                //         create_lyrics(tag, &mut lyric_frames);
                //     }
                // };
                // song.parsed_lyric = lyric_frames
                //     .first()
                //     .map(|lf| Lyric::from_str(&lf.text).ok())
                //     .and_then(|pl| pl);
                // song.lyric_frames = lyric_frames;

                // Get the picture (not necessarily the front cover)
                // let mut picture = tag
                //     .pictures()
                //     .iter()
                //     .find(|pic| pic.pic_type() == PictureType::CoverFront)
                //     .cloned();
                // if picture.is_none() {
                //     picture = tag.pictures().first().cloned();
                // }

                // song.picture = picture;
            }
        }
        Ok(song)
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let p = path.as_ref();
        let directory = Some(get_parent_folder(&p.to_string_lossy()));
        let ext = p.extension().and_then(OsStr::to_str).map(String::from);
        let artist = Some(String::from("Unsupported?"));
        let album = Some(String::from("Unsupported?"));
        let title = p.file_stem().and_then(OsStr::to_str).map(String::from);
        let file_path = Some(p.to_string_lossy().into_owned());
        let wasm_file_path = get_wasm_friendly_path(file_path.clone().unwrap().as_str());
        let duration = Duration::from_secs(0);
        let name = p
            .file_name()
            .and_then(OsStr::to_str)
            .map(std::string::ToString::to_string);
        // let parsed_lyric: Option<Lyric> = None;
        // let lyric_frames: Vec<Lyrics> = Vec::new();
        // let picture: Option<Picture> = None;
        // let album_photo: Option<String> = None;
        let genre = Some(String::from("Unknown"));
        // let last_modified = p.metadata().unwrap().modified().unwrap();
        Self {
            ext,
            file_type: None,
            artist,
            album,
            title,
            file_path,
            wasm_file_path,
            directory,
            duration,
            name,
            genre,
        }
    }

    pub fn artist(&self) -> Option<&str> {
        self.artist.as_deref()
    }

    pub fn set_artist(&mut self, a: &str) {
        self.artist = Some(a.to_string());
    }

    /// Optionally return the song's album
    /// If `None` failed to read the tags
    pub fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    pub fn set_album(&mut self, album: &str) {
        self.album = Some(album.to_string());
    }

    pub fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }

    #[allow(unused)]
    pub fn set_genre(&mut self, genre: &str) {
        self.genre = Some(genre.to_string());
    }

    /// Optionally return the title of the song
    /// If `None` it wasn't able to read the tags
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    pub fn directory(&self) -> Option<&str> {
        self.directory.as_deref()
    }

    pub fn ext(&self) -> Option<&str> {
        self.ext.as_deref()
    }

    pub const fn duration(&self) -> Duration {
        self.duration
    }

    pub fn duration_formatted(&self) -> String {
        Self::duration_formatted_short(&self.duration)
    }

    pub fn duration_formatted_short(d: &Duration) -> String {
        let duration_hour = d.as_secs() / 3600;
        let duration_min = (d.as_secs() % 3600) / 60;
        let duration_secs = d.as_secs() % 60;

        if duration_hour == 0 {
            format!("{duration_min:0>2}:{duration_secs:0>2}")
        } else {
            format!("{duration_hour}:{duration_min:0>2}:{duration_secs:0>2}")
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

pub fn get_parent_folder(filename: &str) -> String {
    let parent_folder: PathBuf;
    let path_old = Path::new(filename);

    if path_old.is_dir() {
        parent_folder = path_old.to_path_buf();
        return parent_folder.to_string_lossy().to_string();
    }
    match path_old.parent() {
        Some(p) => parent_folder = p.to_path_buf(),
        None => parent_folder = std::env::temp_dir(),
    }
    parent_folder.to_string_lossy().to_string()
}

// Don't use tauri-sys or js tauri api to get multiple song asset urls
// Do it here instead when the song is first fetched in backend.
fn get_wasm_friendly_path(path: &str) -> Option<String> {
    let encoded_path = urlencoding::encode(path);
    match env::consts::OS {
        "windows" => {
            let mut windows_path = "https://asset.localhost/".to_string();
            windows_path.push_str(&encoded_path);
            Some(windows_path)
        },
        
        "mac" | "linux" | &_ => {
            let mut unix_path = "asset://localhost/".to_string();
            unix_path.push_str(&encoded_path);
            Some(unix_path)
        }
    }
}
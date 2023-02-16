#![feature(allocator_api)]
#![feature(is_some_and)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::fmt::Debug;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use symphonia::core::codecs::{DecoderOptions, FinalizeResult, CODEC_TYPE_NULL};
use symphonia::core::errors::{Error, Result};
use symphonia::core::formats::{Cue, FormatOptions, FormatReader, SeekMode, SeekTo, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{
    ColorMode, MetadataOptions, MetadataRevision, StandardTagKey, Tag, Value, Visual,
};
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::core::units::{Time, TimeBase};
use symphonia::default::get_probe;
use walkdir::WalkDir;

use crate::song::Song;

#[derive(Copy, Clone)]
struct PlayTrackOptions {
    track_id: u32,
    seek_ts: u64,
}

pub static SUPPORTED_FILETYPES: [&str; 7] = ["wav", "flac", "mp3", "mkv", "alac", "aac", "ogg"];

pub async fn get_all_songs_from_dir_and_subdir(directory_path: &str) -> Vec<Song> {
    let mut vec: Vec<Song> = Vec::new();

    for entry in WalkDir::new(directory_path) {
        let path_str = entry.as_ref().unwrap().path().display().to_string();
        println!("Trying file: {}", path_str);
        let path = Path::new(path_str.as_str());

        if path.is_file() && is_song_supported(path) {
           let song: Song = Song::read_from_path(path, false).unwrap();
           vec.push(song);
        };
    }
    return vec;
}

fn is_song_supported(path: &Path) -> bool {
    match path.extension() {
        Some(ext) if ext == "mp3" => true,
        // Some(ext) if ext == "aiff" => true,
        Some(ext) if ext == "flac" => true,
        Some(ext) if ext == "m4a" => true,
        Some(ext) if ext == "aac" => true,
        // Some(ext) if ext == "opus" => true,
        Some(ext) if ext == "ogg" => true,
        Some(ext) if ext == "wav" => true,
        // Some(ext) if ext == "webm" => true,
        Some(_) | None => false,
    }
}



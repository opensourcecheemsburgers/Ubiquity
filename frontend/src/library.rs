use std::{path::Path, error::Error};

use anyhow::Ok;
use symphonia::default::get_probe;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::Properties;
use walkdir::WalkDir;

use crate::{song::Song, utils::{filetype_supported, is_song_supported}};

#[derive(Properties, PartialEq)]
pub struct Library {
    pub folders: String,
    pub songs: Vec<Song>
}

impl Library {
    pub fn new() -> Self {
        Self { songs: Vec::new(), folders: String::new(), }
    }
    
    pub fn load_songs(path_str: &str) -> Vec<Song> {
        let path: &Path = Path::new(path_str);
        return get_all_songs_from_dir_and_subdir(path.as_ref());
    }

    // pub fn add_folder(&self, folder_path: String) {
    //     self.folders.push(folder_path);
    // }

    // pub fn add_folders(&self, folder_paths: &mut Vec<String>) {
    //     self.folders.append(&mut folder_paths);
    // }
}

pub fn get_all_songs_from_dir_and_subdir(directory_path: &Path) -> Vec<Song> {
    let mut vec: Vec<Song> = Vec::new();

    for entry in WalkDir::new(directory_path) {
        let path_str = entry.as_ref().unwrap().path().display().to_string();
        println!("Trying file: {}", path_str);
        let path = Path::new(path_str.as_str());

        if path.is_file() && is_song_supported(path) {
            let song = Song::read_from_path(path, true).unwrap();
            vec.push(song);
        };
    }
    vec
}


// #[wasm_bindgen(module = "/js/getsongs.js")]
// extern "C" {
//     #[wasm_bindgen(js_name = getSongs, catch)]
//     pub async fn get_songs(path: String) -> Result<JsValue, JsValue>;
// }
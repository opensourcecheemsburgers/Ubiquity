/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::{playlist::Loop, utils::get_app_config_path, FetchError};

use anyhow::{Result, Ok};
use dirs::{audio_dir, config_dir, data_dir};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use futures::{FutureExt, future};
// pub use key::{BindingForEvent, Keys, ALT_SHIFT, CONTROL_ALT, CONTROL_ALT_SHIFT, CONTROL_SHIFT};
use serde::{Deserialize, Serialize};
use std::{fs, path::{PathBuf, Path}};
use tauri_sys::fs::BaseDirectory;

pub const MUSIC_DIR: [&str; 2] = ["~/Music/mp3", "~/Music"];

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LastPosition {
    Yes,
    No,
    Auto,
}

impl std::fmt::Display for LastPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let save_last_position = match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Auto => "auto",
        };
        write!(f, "{save_last_position}")
    }
}
#[derive(Clone, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Config {
    pub music_dir: Vec<String>,
    pub loop_mode: Loop,
    pub volume: i32,
    pub speed: i32,
    pub add_playlist_front: bool,
    pub gapless: bool,
    pub remember_last_played_position: LastPosition,
    pub enable_exit_confirmation: bool,
    pub playlist_select_random_track_quantity: u32,
    pub playlist_select_random_album_quantity: u32,
    pub theme: String,
    // pub keys: Keys,
    pub audio_path: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub data_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        let music_dir = Vec::new();
        // for dir in &MUSIC_DIR {
        //     let absolute_dir = shellexpand::tilde(dir).to_string();
        //     let path = Path::new(&absolute_dir);
        //     if path.exists() {
        //         music_dir.push((*dir).to_string());
        //     }
        // }
        Self {
            music_dir,
            loop_mode: Loop::Queue,
            volume: 70,
            speed: 10,
            add_playlist_front: false,
            gapless: true,
            remember_last_played_position: LastPosition::Auto,
            enable_exit_confirmation: true,
            // keys: Keys::default(),
            theme: "synthwave".to_string(),
            playlist_select_random_track_quantity: 20,
            playlist_select_random_album_quantity: 5,
            audio_path: None,
            config_path: None,
            data_path: None,
        }
    }
}

impl Config {
    pub async fn save(&self) -> Result<()> {
        let mut app_conf_path = self.config_path.clone().unwrap();
        app_conf_path.push("config.toml");
        
        let folder_path = Path::new("ubiquity");
        let file_path = Path::new("ubiquity/config.toml");

        conf_folder_exist(folder_path).await;

        let conf_file_exist = tauri_sys::fs::exists(app_conf_path.as_path(), BaseDirectory::Config).await?;
        if conf_file_exist {
            tauri_sys::fs::remove_file(app_conf_path.as_path(), BaseDirectory::Config).await;
        }

        let string = toml::to_string(self)?;
        let conf_save = tauri_sys::fs::write_text_file(app_conf_path.as_path(), &string, BaseDirectory::Config).await;
        if conf_save.is_err() {
            log::debug!("Err creating config.toml file: {}", conf_save.unwrap_err());
        }

        Ok(())
    }

    pub async fn load(&mut self, mut path: &Path) -> Result<()> {
        conf_folder_exist(path).await;

        let mut config_path = tauri_sys::path::config_dir().await.unwrap();
        let mut data_path = tauri_sys::path::data_dir().await.unwrap();
        let audio_path = tauri_sys::path::audio_dir().await.unwrap();

        config_path.push("ubiquity/");
        data_path.push("ubiquity/");

        log::debug!("Config path: {}", config_path.to_string_lossy());
        log::debug!("Data path: {}", data_path.to_string_lossy());

        self.audio_path = Some(audio_path);
        self.data_path = Some(data_path);
        self.config_path = Some(config_path);

        self.save().await?;

        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(path))
            .extract()?;
        *self = config;
        Ok(())
    }
}

async fn conf_folder_exist(folder_path: &Path) {
    let exist = tauri_sys::fs::exists(folder_path, BaseDirectory::Config).await.unwrap();

    if !exist {
        tauri_sys::fs::create_dir(folder_path, BaseDirectory::Config).await;
    }
}

async fn conf_file_exist(file_path: &Path) {
    tauri_sys::fs::exists(file_path, BaseDirectory::Config).await.unwrap();
}
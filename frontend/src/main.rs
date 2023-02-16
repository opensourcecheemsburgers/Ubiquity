mod components;
mod pages;
mod song;
mod library;
mod player;
mod playlist;
mod config;
mod utils;
mod theme;

use core::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::rc::Rc;
use std::vec;

use components::{ThemeCtx, header};
use config::{Config};
use library::Library;
use pages::{welcome, home, about, settings};
use player::UbiquityPlayer;
use playlist::Playlist;
use song::Song;
use tauri_sys::dialog::FileDialogBuilder;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::player::rust::Player;

use yew::prelude::*;
use yew::{html, Html};

use dirs::{audio_dir, data_dir, config_dir};
use crate::player::PlayerTrait;

#[wasm_bindgen(module = "/js/getsongs.js")]
extern "C" {
    #[wasm_bindgen(js_name = getSongs, catch)]
    pub async fn get_songs(path: String) -> Result<JsValue, JsValue>;
}

async fn get_songs_using_js(path: String) -> String {
    get_songs(path).await.unwrap().as_string().unwrap()
}

async fn choose_folder() -> Result<Option<PathBuf>, FetchError> {
    let folder_fut = FileDialogBuilder::new()
        .set_title("Select Music Library for Ubiquity")
        .pick_folder().await;

    match folder_fut {
        Ok(selected_folder) => {
            Ok(selected_folder)
        },
        Err(error) => {
            print!("Result: {}", error);
            Err(FetchError { fetch_error: error.to_string()})
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    fetch_error: String,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.fetch_error, f)
    }
}

pub enum LibraryState<> {
    NoLibrarySelected,
    FolderSelected,
    FetchingLibrary,
    LibraryLoaded,
}

pub enum Page<> {
    Welcome,
    Home,
    About,
    Settings
}

pub enum Msg {
    SetLibraryState(LibraryState<>),
    SetLibraryFolder(),
    FetchLibrary(String),
    SetLibrary(Vec<Song>),
    
    SetTheme(String),
    OpenPage(Page<>),
    AddSong(Song),

    SetPlaylist(Playlist),
    SetPlayer(UbiquityPlayer),

    SetConfig(Config),
}

pub struct App {
    library: Library,
    library_state: LibraryState<>,
    current_page: Page<>,
    config: Config,
    playlist: Playlist,
    player: Option<UbiquityPlayer>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        wasm_logger::init(wasm_logger::Config::default());

        let config: Config = Config::default();
        let library = Library::new();
        let mut library_state = LibraryState::NoLibrarySelected;
        let mut current_page = Page::Welcome;
        let playlist: Playlist = Playlist::default();
        let player: Option<UbiquityPlayer> = None;

        ctx.link().send_future_batch(async move {
            let mut config_path = tauri_sys::path::config_dir().await.unwrap();
            config_path.push("ubiquity");

            let mut config = Config::default();
            config.load(config_path.as_path()).await;

            let conf_clone = config.clone();

            let playlist: Playlist = Playlist::new(&conf_clone).await.unwrap();
            let player: UbiquityPlayer = UbiquityPlayer::new(&conf_clone, playlist.clone());

            vec![Msg::SetConfig(config), Msg::SetPlaylist(playlist), Msg::SetPlayer(player)]
        });

        Self {
            library_state,
            library,
            config,
            current_page,
            playlist,
            player,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLibraryState(fetch_state) => {
                self.library_state = fetch_state;
                true
            }
            Msg::SetLibraryFolder() => {
                ctx.link().send_future_batch(async {
                    let folder = choose_folder().await;

                    let path = folder.unwrap().unwrap().as_path().to_string_lossy().to_string();
                    log::info!("Path chosen: {}", path.clone());
                    let res = get_songs_using_js(path).await;
                    let songs: Vec<Song> = serde_json::from_str(&res).unwrap();
                    log::info!("Received {} songs from backend", songs.len());

                    vec![Msg::SetLibrary(songs), Msg::SetLibraryState(LibraryState::LibraryLoaded), Msg::OpenPage(Page::Home)]
                });
                ctx.link().send_message(Msg::SetLibraryState(LibraryState::FetchingLibrary));
                true
            }
            Msg::FetchLibrary(path) => {
                ctx.link().send_future_batch(async move {
                    let songs: Vec<Song> = Library::load_songs(path.as_str());
                    log::info!("Received {} songs from backend", songs.len());
                    vec![Msg::SetLibrary(songs), Msg::SetLibraryState(LibraryState::LibraryLoaded)]
                });
            true
            },
            Msg::SetTheme(theme) => {
                self.config.theme = theme.clone();
                true
            },
            Msg::SetLibrary(songs) => {
                self.library.songs = songs;
                true
            },
            Msg::OpenPage(page) => {
                self.current_page = page;
                true
            }
            Msg::AddSong(song) => {
                let player = self.player.as_mut().unwrap();
                player.playlist.set_current_track(Some(&song));
                player.start_play();
                player.set_volume(100);
                false
            },
            Msg::SetPlaylist(playlist) => {
                self.playlist = playlist;
                false
            },
            Msg::SetPlayer(player) => {
                self.player = Some(player);
                false
            },
            Msg::SetConfig(config) => {
                self.config = config;
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &&self.current_page {
            Page::Welcome => welcome(ctx, &self.config.theme),
            Page::Home => home(ctx, &self.config.theme, &self.library.songs),
            Page::About => about(ctx, &self.config.theme),
            Page::Settings => settings(ctx, &self.config.theme),
        }    
    
    }
}

pub fn theme_callback(ctx: &Context<App>, theme_name: String) -> Callback<MouseEvent> {
    ctx.link().callback(move |click: MouseEvent| Msg::SetTheme(theme_name.clone()))
}

fn main() {
    yew::Renderer::<App>::new().render();
}
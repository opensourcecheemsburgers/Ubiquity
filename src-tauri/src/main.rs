#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod sherlock;
mod song;

use std::future::IntoFuture;
use native_dialog::{FileDialog, MessageDialog, MessageType};

use sherlock::{get_all_songs_from_dir_and_subdir};
use song::Song;


#[tauri::command]
async fn get_songs(path: String) -> Result<String, String> {
    let songs: Vec<Song> = get_all_songs_from_dir_and_subdir(path.as_str()).await;
    Ok(serde_json::to_string(&songs).unwrap())
}

// async fn get_songs_using_js() -> String {
//     let js_future = JsFuture::from(getSongs("/home/winston/HDD/Music/Avenged Sevenfold/".to_string()));
//     let result = js_future.await;
//     match result {
//         Ok(songs) => {
//             songs.as_string().unwrap()
//         }
//         Err(error) => {
//             error.as_string().unwrap()
//         }
//     }
// }

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_songs])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    // block_on(get_songs_using_js());
}

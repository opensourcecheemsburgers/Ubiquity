use std::path;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{components::{Footer, ArtistCard, header, table}, App, Msg, theme_callback, song::Song, config::Config, player::UbiquityPlayer, playlist::Playlist};


pub fn home(ctx: &Context<App>, theme: &str, songs: &Vec<Song>) -> Html {
    html! {
        <div data-theme={theme.to_owned()}>
            {header(ctx)}
            {table(ctx, songs)}
        </div>
    }
}

pub fn welcome(ctx: &Context<App>, theme: &str) -> Html {
    html! {
        <div data-theme={theme.to_owned()}>
        {header(ctx)}
        <div class="hero min-h-[calc(100vh-64px)] bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-xl">
                    <h1 class="text-6xl font-bold font-display">{"Ubiquity"}</h1>
                    <p class="text-xl py-6 font-mono">{"A cross-platform music player for Windows, Mac & Linux."}</p>
                    <button onclick={ctx.link().callback(|_| Msg::SetLibraryFolder())} class="btn btn-primary btn-outline">{"Select Songs"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::TestPlayer())} class="btn btn-primary btn-outline">{"Test wasm audio"}</button>
                    <audio controls={true}>
                    <source src="https://www.kozco.com/tech/piano2-CoolEdit.mp3" type="audio/mpeg"/>
                    <source src="horse.mp3" type="audio/mpeg"/>
                    {"Your browser does not support the audio tag."}
                  </audio> 
                </div>
            </div>
        </div>
    </div>
    }
}

pub fn about(ctx: &Context<App>, theme: &str) -> Html {
    html! {
        <div data-theme={theme.to_owned()} class="select-none">
            {header(ctx)}
            <div class="hero min-h-[calc(100vh-64px)] bg-base-200">
                <div class="hero-content">
                    <article class="prose-xl prose-a:text-primary hover:prose-a:text-primary-focus font-mono cursor-default">
                        <h1 class="font-display">{"Ubiquity"}</h1>
                        <p class="text-2xl">
                            {"An open-source, cross-platform music player made with Tauri."}
                        </p>
                        <p class="text-2xl">
                            {"Ubiquity is written in Rust and its frontend is built with Yew and DaisyUI."}
                        </p>                        
                        <h2 class="font-display">{"Links"}</h2>
                        <div class="not-prose flex justify-start space-x-5">
                        <a href="https://github.com/opensourcecheemsburgers/Ubiquity" target="_blank" class="btn btn-lg btn-outline gap-2">{"Source Code"}</a>
                        <a href="https://tauri.app/" target="_blank" class="btn btn-lg btn-outline">{"Tauri"}</a>
                        <a href="https://yew.rs/" target="_blank" class="btn btn-lg btn-outline">{"Yew"}</a>
                        <a href="https://daisyui.com/" target="_blank" class="btn btn-lg btn-outline">{"DaisyUI"}</a>
                        <a href="https://daisyui.com/" target="_blank" class="btn btn-lg btn-outline">{"Termusic"}</a>
                        </div>
                    </article>
                </div>
            </div>
        </div>
    }
}


pub fn settings(ctx: &Context<App>, theme: &str) -> Html {    
    html! {
        <div data-theme={theme.to_owned()}>
            {header(ctx)}
            <p>{"Settings"}</p>
        </div>
    }
}

// fn open_link(ctx: &Context<App>, link: String) -> Callback<MouseEvent> {
//     ctx.link().callback(move |mouse_event: MouseEvent| Msg::OpenLink(link.to_owned()))
// }
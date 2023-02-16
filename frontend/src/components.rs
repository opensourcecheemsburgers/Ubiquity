use std::rc::Rc;

use yew::{prelude::*, virtual_dom::VNode};

use crate::{song::{Song, self}, theme::Theme, theme_callback, App, Msg, Page};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}

/// Controlled Text Input Component
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="box-border h-36 py-3 px-6 w-6/12 bg-base-200 rounded-t-3xl flex flex-col justify-between">
            <div class="flex justify-between">
                <div class="flex items-center">
                    <img class="h-20 w-20 rounded-xl" src="" />
                    <div class="h-24 py-3 flex flex-col justify-between">
                        <p class="text- text-3xl font-bold font-sans text-left pl-6">{"Trivium"}</p>
                        <p class="text-left pl-6">{"Amongst the Shadows & the Stones"}</p>
                    </div>
                </div>
                <div class="flex items-center">
                    <button class="btn btn-square btn-primary">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="19 20 9 12 19 4 19 20"></polygon><line x1="5" y1="19" x2="5" y2="5"></line></svg>
                    </button>
                    <button class="btn btn-square btn-primary mx-2">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="6" y="4" width="4" height="16"></rect><rect x="14" y="4" width="4" height="16"></rect></svg>
                    </button>
                    <button class="btn btn-square btn-primary">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 4 15 12 5 20 5 4"></polygon><line x1="19" y1="5" x2="19" y2="19"></line></svg>
                    </button>
                </div>
            </div>
            <div class="flex items-center pt-1">
                <p class="text-left">{"0:00"}</p>
                <input type="range" min="0" max="1000" value="40" class="range range-xs range-secondary mx-4" />
                <p class="text-left">{"0:00"}</p>
            </div>
        </div>
    }
}


pub fn header(ctx: &Context<App>) -> Html {
    let header = html! {
        <div class="navbar bg-base-100">
        <div class="navbar-start">
        <label htmlFor="my-drawer" class="btn btn-square btn-ghost">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
        </label>
        </div>
        <div class="navbar-center">
        <p class="font-display text-2xl">{"Ubiquity"}</p>
        </div>
        <div class="navbar-end">
        <button class="btn btn-ghost btn-circle">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
        </button>
  
        <div class="dropdown dropdown-end">
        <label tabindex="0" class="btn btn-ghost rounded-btn">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
        </label>
        <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-100 rounded-box w-52 mt-4">
          <li><a onclick={theme_callback(ctx, "cupcake".to_string())}>{"Cupcake"}</a></li>
          <li><a onclick={theme_callback(ctx, "synthwave".to_string())}>{"Synthwave"}</a></li>
          <li><a onclick={theme_callback(ctx, "light".to_string())}>{"Light"}</a></li>
          <li><a onclick={theme_callback(ctx, "retro".to_string())}>{"Retro"}</a></li>
          <li><a onclick={theme_callback(ctx, "aqua".to_string())}>{"Aqua"}</a></li>
          <li><a onclick={ctx.link().callback(|_| Msg::OpenPage(Page::About))}>{"About"}</a></li>
        </ul>
      </div>
        </div>
        </div>
      };

      header
}

pub fn table(ctx: &Context<App>, songs: &Vec<Song>) -> Html {
    let mut songs_html: Vec<Html> = Vec::new(); 
    
    for (index, song) in songs.iter().enumerate() {
        let song_clone = song.clone();
        let callback: Callback<MouseEvent> = ctx.link().callback(move |click: MouseEvent| Msg::AddSong(song_clone.clone()));
        let html = song.render_table_item(index, callback);
        songs_html.push(html);
    };

    // let s = songs.iter().map(|song| {song.render_table_item()});

    html! {
        <div class="scrollbar overflow-y-scroll">
            <table class="table table-zebra w-full">
                <thead>
                    <tr>
                        <th>{"Title"}</th>
                        <th>{"Artist"}</th>
                        <th>{"Album"}</th>
                        <th>{"Length"}</th>
                        // <th>{"Sample Rate"}</th>
                        // <th>{"Bit Rate"}</th>
                        // <th>{"Bit Depth"}</th>
                        <th>{"File Type"}</th>
                        <th>{"Save Path"}</th>
                    </tr>
                </thead>
                { for songs_html }
            </table>
        </div>
    }
}

#[function_component(ThemeCtx)]
pub fn theme_ctx(props: &Props) -> Html {
    let theme = use_memo(|_| Theme {
        name: "cupcake".to_owned(),
    }, ());

    html! {
        <ContextProvider<Rc<Theme>> context={theme}>
            { for props.children.iter() }
        </ContextProvider<Rc<Theme>>>
    }
}

#[function_component(ArtistCard)]
pub fn artist_card() -> Html {
    let input_ref: NodeRef = NodeRef::default();

    html! {
        <div class="card w-11/12 sm:w-9/12 md:w-9/12
        lg:w-8/12 xl:w-7/12 2xl:w-4/12 bg-base-100 drop-shadow-2xl rounded-3xl">
            <div class="card-body">
                <p class="card-title text-4xl font-display pb-2">{"Trivium"}</p>
                <label for="check" class="togButton" ref={input_ref}>{"Toggle"}</label>
                <Album />
                <Album />
                <Album />
                <Album />
            </div>
        </div>
    }
}

#[function_component(Album)]
fn album() -> Html {
    html! {
        <div class="flex items-center bg-base-300 p-4 rounded-3xl">
            <div class="box-content h-24 w-24">
                <img class="rounded-2xl"/>
            </div>
            <div>
                <div class="flex flex-col font-light font-mono pt-1">
                    // <Song />
                </div>
            </div>
        </div>
    }
}

#[function_component(SongItem)]
fn song_item(song: &Song) -> Html {
    html! {
        <div class="flex py-1">
            <p class="text-left pl-3">{song.title()}</p>
            <p class="text-right pr-3">{song.duration_formatted()}</p>
        </div>
    }
}

#[derive(PartialEq)]
pub enum BtnSize {
    VerySmall,
    Small,
    Medium,
    Large,
    VeryLarge
}

impl Default for BtnSize {
    fn default() -> Self {
        BtnSize::Medium
    }
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub on_click: Callback<MouseEvent>,
    pub is_link: bool,
    pub btn_title: AttrValue,
    pub btn_size: BtnSize
}

// #[function_component(Button)]
// fn button(props: &ButtonProps) -> Html {
//     match props.btn_size {
//         BtnSize::Large => 
//     }

//     let class_str = String::from("btn").extend(iter);
    
//     html! {
//         <button 
//             onclick={props.on_click.clone()}
//             class="btn btn-lg btn-outline">
//             {props.btn_title.clone()}
//         </button>
//     }
    
    
//     // html! {
//     //     <a href="https://github.com/opensourcecheemsburgers/Ubiquity" target="_blank" class="btn btn-lg btn-outline gap-2">{"Source Code"}</a>
//     // }
// }
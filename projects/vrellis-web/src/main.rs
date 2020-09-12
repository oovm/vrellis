#![recursion_limit = "1024"]
mod ascii_view;
mod braille_view;
mod global;

pub use crate::global::{format_image_size, GlobalSettings, Scene};
use image::{load_from_memory_with_format, DynamicImage, ImageFormat};
use std::fmt::{self, Debug, Formatter};
use yew::{
    format::Json,
    html,
    services::{
        reader::{FileData, ReaderService, ReaderTask},
        storage::Area,
        DialogService, StorageService,
    },
    ChangeData, Component, ComponentLink, Html, ShouldRender,
};

const KEY: &str = "ascii-art";

#[derive(Debug)]
pub enum Event {
    SwitchTo(Scene),
    Files(ChangeData),
    FilesLoaded(FileData),
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    tasks: Vec<ReaderTask>,
    state: GlobalSettings,
    ascii_image: Option<DynamicImage>,
    braille_image: Option<DynamicImage>,
    emoji_image: Option<DynamicImage>,
}

impl Debug for Model {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("GlobalSettings")
            .field("scene", &self.state)
            .field("tasks", &self.tasks)
            .field("ascii.image", &format_image_size(&self.ascii_image))
            .field("braille.image", &format_image_size(&self.braille_image))
            .field("emoji.image", &format_image_size(&self.emoji_image))
            .finish()
    }
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let state = match storage.restore(KEY) {
            Json(Ok(state)) => state,
            _ => GlobalSettings::default(),
        };
        Self { link, storage, tasks: vec![], state, ascii_image: None, braille_image: None, emoji_image: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::SwitchTo(scene) => {
                self.state.scene = scene;
                self.storage.store(KEY, Json(&self.state))
            }
            Event::Files(ChangeData::Files(f)) => {
                let task = ReaderService::new().read_file(f.get(0).unwrap(), self.link.callback(Event::FilesLoaded)).unwrap();
                self.tasks.push(task)
            }
            Event::FilesLoaded(data) => {
                let img = match load_from_memory_with_format(&data.content, ImageFormat::Png) {
                    Ok(o) => o,
                    Err(e) => {
                        DialogService::alert(&format!("{}", e));
                        return false;
                    }
                };
                match self.state.scene {
                    Scene::AsciiArt => self.ascii_image = Some(img),
                    Scene::BrailleArt => self.braille_image = Some(img),
                    Scene::EmojiArt => self.emoji_image = Some(img),
                }
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.state.scene {
            Scene::AsciiArt => html! {
            <>
                {self.navbar_view()}
                {self.ascii_view()}
            </>
            },
            Scene::BrailleArt => html! {
            <>
                {self.navbar_view()}
                {self.braille_view()}
            </>
            },
            Scene::EmojiArt => html! {
            <>
                {self.navbar_view()}
            </>
            },
        }
    }
}

impl Model {
    pub fn navbar_view(&self) -> Html {
        html! {
        <nav class="navbar ">
            <div class="container">
                <div class="navbar-brand">
                    <a class="navbar-item">
                        <img src="https://bulma.io/images/bulma-type-white.png" alt="Logo"/>
                    </a>
                    <span class="navbar-burger burger">
                      <span></span>
                      <span></span>
                      <span></span>
                    </span>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-end">
                    {self.switch_to_ascii()}
                    {self.switch_to_braille_art()}
                    {self.switch_to_emoji()}
                    <iframe id="github-star"
                        src="https://ghbtns.com/github-btn.html?user=GalAster&repo=ascii-art&type=star&count=true&size=large"
                        frameborder="0" scrolling="0" width="120" height="50" title="GitHub" loading="lazy"
                    />
                    </div>
                </div>
            </div>
        </nav>
        }
    }
    fn switch_to_ascii(&self) -> Html {
        let class = match self.state.scene {
            Scene::AsciiArt => "navbar-item is-active",
            _ => "navbar-item",
        };
        html! {
            <a class=class id="title"
                onclick=self.link.callback(|_| Event::SwitchTo(Scene::AsciiArt))
            >
            {"AsciiArt"}
            </a>
        }
    }
    fn switch_to_emoji(&self) -> Html {
        let class = match self.state.scene {
            Scene::EmojiArt => "navbar-item is-active",
            _ => "navbar-item",
        };
        html! {
            <a class=class id="title"
                onclick=self.link.callback(|_| Event::SwitchTo(Scene::EmojiArt))
            >
            {"EmojiArt"}
            </a>
        }
    }
    fn switch_to_braille_art(&self) -> Html {
        let class = match self.state.scene {
            Scene::BrailleArt => "navbar-item is-active",
            _ => "navbar-item",
        };
        html! {
            <a class=class id="title"
                onclick=self.link.callback(|_| Event::SwitchTo(Scene::BrailleArt))
            >
            {"BrailleArt"}
            </a>
        }
    }
}

impl Model {
    pub fn image_upload_bottom(&self) -> Html {
        html! {
        <div class="field">
            <div class="file is-info">
                <label class="file-label">
                    <input class="file-input" type="file" multiple=true
                        onchange=self.link.callback(|input: ChangeData| Event::Files(input))
                    />
                    <span class="file-cta">
                      <span class="file-icon"><i class="fa fa-upload"></i></span>
                      <span class="file-label">{"Choose a image…"}</span>
                    </span>
                </label>
            </div>
        </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

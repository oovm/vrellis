#![recursion_limit = "1024"]

mod global;
use crate::global::GlobalSettings;
use image::{load_from_memory_with_format, ImageFormat};
use std::{num::ParseIntError, str::FromStr};
use vrellis_core::{Vrellis, VrellisAlgorithm};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        reader::{FileData, ReaderService, ReaderTask},
        storage::Area,
        DialogService, StorageService,
    },
    Component, ComponentLink, Html, ShouldRender,
};
mod form;

const KEY: &str = "vrellis art";

pub enum Event {
    Points(ChangeData),
    Steps(ChangeData),
    Shape(ChangeData),
    LineWidth(ChangeData),
    AntiAliased(ChangeData),
    Files(ChangeData),
    FilesLoaded(FileData),
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    tasks: Vec<ReaderTask>,
    image: Vec<u8>,
    state: GlobalSettings,
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
        Self { link, storage, tasks: vec![], image: vec![], state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Points(ChangeData::Value(v)) => {
                if let Ok(o) = u32::from_str(&v) {
                    self.state.points = o
                }
            }
            Event::Steps(ChangeData::Value(v)) => {
                if let Ok(o) = u32::from_str(&v) {
                    self.state.steps = o
                }
            }
            Event::LineWidth(ChangeData::Value(v)) => {
                if let Ok(o) = f32::from_str(&v) {
                    self.state.line_width = o
                }
            }
            Event::Shape(ChangeData::Select(s)) => {

            },
            Event::AntiAliased(_) => self.state.anti_aliased = !self.state.anti_aliased,
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
            }
            _ => (),
        }
        self.storage.store(KEY, Json(&self.state));
        return true;
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <main class="container-fluid">
            <div class="page-header">
                <h1>{"QR Image Embed"}</h1>
                <span>
                <iframe
                    src="https://ghbtns.com/github-btn.html?user=GalAster&repo=qr-image&type=star&count=true&size=large"
                    frameborder="0" scrolling="0" width="170" height="30" title="GitHub" loading="lazy"
                />
                </span>
            </div>
            {self.form_view()}
        </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

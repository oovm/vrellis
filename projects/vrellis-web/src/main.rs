#![recursion_limit = "1024"]

mod global;
use crate::global::GlobalSettings;
use image::{load_from_memory_with_format, DynamicImage, ImageFormat};
use std::str::FromStr;
use vrellis_core::VrellisShape;
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
    Refresh,
    Play(ChangeData),
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    tasks: Vec<ReaderTask>,
    image: DynamicImage,
    state: GlobalSettings,
    output: Vec<String>,
    output_index: usize,
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
        let default = include_bytes!("github.png") as &[u8];
        let image = load_from_memory_with_format(default, ImageFormat::Png).unwrap();
        Self { link, storage, tasks: vec![], image, state, output: vec![], output_index: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Steps(ChangeData::Value(v)) => {
                if let Ok(o) = u32::from_str(&v) {
                    self.state.steps = o
                }
                self.storage.store(KEY, Json(&self.state));
            }
            Event::Points(ChangeData::Value(v)) => {
                if let Ok(o) = u32::from_str(&v) {
                    self.state.points = o
                }
                self.storage.store(KEY, Json(&self.state));
            }
            Event::LineWidth(ChangeData::Value(v)) => {
                if let Ok(o) = f32::from_str(&v) {
                    self.state.line_width = o
                }
                self.storage.store(KEY, Json(&self.state));
            }
            Event::Shape(ChangeData::Select(s)) => {
                let shape = match s.value().as_ref() {
                    "3" => VrellisShape::Triangle,
                    "4" => VrellisShape::Square,
                    _ => VrellisShape::Circle,
                };
                self.state.convex_shape = shape;
                self.storage.store(KEY, Json(&self.state));
            }
            Event::AntiAliased(_) => {
                self.state.anti_aliased = !self.state.anti_aliased;
                self.storage.store(KEY, Json(&self.state));
            }
            Event::Files(ChangeData::Files(f)) => {
                let task = ReaderService::new().read_file(f.get(0).unwrap(), self.link.callback(Event::FilesLoaded)).unwrap();
                self.tasks.push(task)
            }
            Event::FilesLoaded(data) => {
                match load_from_memory_with_format(&data.content, ImageFormat::Png) {
                    Ok(o) => self.image = o,
                    Err(e) => {
                        DialogService::alert(&format!("{}", e));
                        return false;
                    }
                };
            }
            Event::Refresh => {
                let ctx = self.state.build();
                let mut state = ctx.render(self.image.clone());
                state.steps(self.state.steps);
                self.output = state.draw_svg_steps();
                self.output_index = self.output.len()-1
            }
            Event::Play(ChangeData::Value(v)) => {
                if let Ok(o) = usize::from_str(&v) {
                    self.output_index = o
                }
            }
            _ => return false,
        }
        return true;
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <main class="container-fluid">
            <div class="page-header">
                <h1>{"Vrellis String Art"}</h1>
                <span>
                <iframe
                    src="https://ghbtns.com/github-btn.html?user=GalAster&repo=vrellis&type=star&count=true&size=large"
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

use super::*;

impl Model {
    pub fn ascii_view(&self) -> Html {
        let out = format!("{:#?}", self);
        html! {out}
    }
}

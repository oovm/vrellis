use crate::{Event, Model};

use vrellis_core::VrellisShape;
use yew::prelude::*;

impl Model {
    pub fn output_view(&self) -> Html {
        if self.output.is_empty() {
            return self.empty_output_view()
        }
        let t = yew::utils::document().create_element("div").unwrap();
        match self.output.get(self.output_index) {
            None => {
                
                return self.empty_output_view()
            }
            Some(s) => {
                t.set_inner_html(s)
            }
        }
        let out = Html::VRef(t.first_child().unwrap().into());
        html! {<>
        <div class="form-group">
            <label class="col-sm-2">{"Output:"}</label>
            <div class="col-sm-10">{out}</div>
        </div>
        <div class="form-group">
            <label class="col-sm-2">{"Steps:"}</label>
            <div class="col-sm-9">
                <div class="form-control-static">
                    <input type="range" min=0 max=self.output.len()-1 step=1
                        value=self.output_index
                        onchange=self.link.callback(|input: ChangeData| Event::Play(input))
                    />
                </div>
            </div>
            <label class="col-sm-1">{self.output_index}</label>
        </div>
        </>}
    }

    pub fn form_view(&self) -> Html {
        html! {
        <form class="form-horizontal">
            {self.output_view()}
            <div class="form-group">
                <label class="col-sm-2">{"Image:"}</label>
                <div class="col-sm-8">
                    <input type="file" multiple=true
                        onchange=self.link.callback(|input: ChangeData| Event::Files(input))
                    />
                </div>
                <button class="col-sm-2 btn btn-primary" type="button"
                    onclick=self.link.callback(|_| Event::Refresh)
                >{"Render!"}</button>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Iterations:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="100" max="4000" step="100"
                            value=self.state.steps
                            onchange=self.link.callback(|input: ChangeData| Event::Steps(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{self.state.steps}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Points:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="50" max="500" step="50"
                            value=self.state.points
                            onchange=self.link.callback(|input: ChangeData| Event::Points(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{self.state.points}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Shape:"}</label>
                <div class="col-sm-10">
                    <select class="form-control"
                        value=self.shape_view()
                        onchange=self.link.callback(|input: ChangeData| Event::Shape(input))
                    >
                        <option value="3">{"Triangle"}</option>
                        <option value="4">{"Square"}</option>
                        <option value="0">{"Circle"}</option>
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"AntiAliased:"}</label>
                <div class="col-sm-10">
                    <input type="checkbox"
                        checked=self.state.anti_aliased
                        onchange=self.link.callback(|input: ChangeData| Event::AntiAliased(input))
                    />
                 </div>
            </div>
        </form>
        }
    }
}

impl Model {
    fn shape_view(&self) -> u32 {
        match self.state.convex_shape {
            VrellisShape::Triangle => 3,
            VrellisShape::Square => 4,
            VrellisShape::Circle | _ => 0,
        }
    }
    fn empty_output_view(&self) -> Html {
        html! {
        <div class="form-group">
            <label class="col-sm-2">{"Output:"}</label>
            <div class="col-sm-10"></div>
        </div>
        }
    }
}

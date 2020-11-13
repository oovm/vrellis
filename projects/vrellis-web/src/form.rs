use crate::{Event, Model};
use image::{imageops::FilterType, GenericImageView, ImageFormat, ImageOutputFormat};
use yew::prelude::*;
use vrellis_core::VrellisAlgorithm;

impl Model {
    pub fn qr_code_view(&self) -> Html {
        return html! {
        <div class="form-group">
            <label class="col-sm-2">{"QR_CODE:"}</label>
            <div class="col-sm-10">{"qr"}</div>
        </div>
        };
    }

    pub fn form_view(&self) -> Html {
        html! {
        <form class="form-horizontal">
            {self.qr_code_view()}
            <div class="form-group">
                <label class="col-sm-2">{"Text:"}</label>
                <div class="col-sm-10">
                    <textarea class="form-control" rows="3"
                        value="self.input"
                        oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Image:"}</label>
                <div class="col-sm-10">
                    <input type="file" multiple=true
                        onchange=self.link.callback(|input: ChangeData| Event::Files(input))
                    />
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"QR Version:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="1" max="10" step="1"
                            value="self.format_qr_version()"
                            onchange=self.link.callback(|input: ChangeData| Event::QRVersion(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{"self.format_qr_version()"}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Output Size:"}</label>
                <div class="col-sm-9">
                    <div class="form-control-static">
                        <input type="range" min="80" max="640" step="20"
                            value="self.output_size"
                            onchange=self.link.callback(|input: ChangeData| Event::OutputSize(input))
                        />
                    </div>
                </div>
                <label class="col-sm-1">{"self.output_size"}</label>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"EC Level:"}</label>
                <div class="col-sm-10">
                    <select class="form-control"
                        value="self.format_ec_level()"
                        onchange=self.link.callback(|input: ChangeData| Event::ECLevel(input))
                    >
                        <option value="L">{"L"}</option>
                        <option value="Q">{"Q"}</option>
                        <option value="M">{"M"}</option>
                        <option value="H">{"H"}</option>
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Enhanced:"}</label>
                <div class="col-sm-10">
                    <input type="checkbox"
                        checked=self.algorithm_view()
                        onchange=self.link.callback(|input: ChangeData| Event::EnhanceMode(input))
                    />
                 </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Background:"}</label>
                <div class="col-sm-10">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::LightColor(input))
                        />
                    </div>
                </div>
            </div>
            <div class="form-group">
                <label class="col-sm-2">{"Foreground:"}</label>
                <div class="col-sm-10">
                    <div class="form-control-static">
                        <input type="color"
                            onchange=self.link.callback(|input: ChangeData| Event::DarkColor(input))
                        />
                    </div>
                </div>
            </div>
        </form>
        }
    }
}


impl Model {
    fn algorithm_view(&self)->bool {
        match self.state.algorithm {
            VrellisAlgorithm::NonRendered => false,
            VrellisAlgorithm::ThinLine => false,
            VrellisAlgorithm::AntiAliased => true,
        }
    }
}
use super::*;

impl Model {
    pub fn braille_view(&self) -> Html {
        html! {
                    <main class="column">
        <section class="section" id="typography">
            <h1 class="title"> {"Typography"} </h1>
            <hr/>
            <div class="column">
                        <h1 class="subtitle is-1"> {"Subtitle 1"} </h1>
                    </div>
        </section>
        {self.image_upload_bottom()}
        </main>
                }
    }
}

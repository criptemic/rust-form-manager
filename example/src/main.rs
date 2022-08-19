use ::form::form::Form;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!(
        <main class="bg-grey-100 h-screen">
            <div class="mx-auto max-w-screen-2xl">
             <Form />
            </div>
        </main>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

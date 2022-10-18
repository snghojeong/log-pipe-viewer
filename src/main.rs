use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World! This is log viewer!" }</h1>
        <h1>{ "Autor: snghojeong" }</h1>
        <h1>{ "Date: YYYY-MM-DD" }</h1>
        <h1>{ "File:" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}

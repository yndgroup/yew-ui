use web_sys::console::log;
use yew::prelude::*;

use yew_ui::components::{theme::TypeStyle, Button};
use gloo::console::log;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="container">
        <Button text="Primary 按钮" click={|_| log!("点击了确认按钮")} r#type={TypeStyle::Primary}/>
        <Button text="Warning 按钮" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
        <Button text="Danger 按钮" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Danger}/>
        <Button text="Success 按钮" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Success}/>
        <Button text="Info 按钮" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Info}/>
        <Button text="默认按钮" click={|_| log!("点击了取消按钮")} />
        <h1>{"Welcome to Tauri + Yew"}</h1>
    </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

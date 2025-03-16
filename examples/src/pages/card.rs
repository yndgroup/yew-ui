use yew::prelude::*;
use yew_ui::components::prelude::*;

#[function_component(PageCard)]
pub fn page_card() -> Html {
    html! {
        <div class="container">
            <h1 class="text-2xl">{"Card"}</h1>
            <Card>{"组件是一个卡片组件，用于展示内容。"}</Card>
        </div>
    }
}

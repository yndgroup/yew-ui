use yew::prelude::*;
use yew_ui::components::prelude::*;

#[function_component(PageBorder)]
pub fn page_border() -> Html {
    return html! {
        <div class="container">
            <h1 class="text-2xl">{"Border"}</h1>
            <p>{"我们对边框进行统一规范，可用于按钮、卡片、弹窗等组件里。"}</p>
            <Container border={Border{width: SetBorderWidth::default()}.to_string()}>
                {1111}
            </Container>
        </div>
    };
}

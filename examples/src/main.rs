use yew::prelude::*;
use yew_ui::components::prelude::*;
use yew_ui::gloo::console::log;

use yew_ui::toolkit::math::add;

#[function_component(App)]
fn app() -> Html {
    log!(add(1, 2));
    html! {
        <main class="p-5">
        <h1>{"Welcome Yew + YewUI"}</h1>
        <dl>
            <dt class="py-2">{"基础按钮"}</dt>
            <dd>
                <Button text="Primary 按钮" custom_class="mr-2" click={|_| log!("点击了确认按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Warning 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Danger 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Danger}/>
                <Button text="Success 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Success}/>
                <Button text="Info 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Info}/>
                <Button text="默认按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} />
            </dd>
            <dt class="py-2">{"块按钮"}</dt>
            <dd>
                <Button text="Primary 按钮" block=true custom_class="mb-2" click={|_| log!("点击了确认按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Warning 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Danger 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Danger}/>
                <Button text="Success 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Success}/>
                <Button text="Info 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Info}/>
                <Button text="默认按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} />
            </dd>
        </dl>
    </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

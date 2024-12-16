use yew::prelude::*;
use yew_ui::components::prelude::*;
use yew_ui::gloo::console::log;

use yew_ui::toolkit::math::add;

#[function_component(App)]
fn app() -> Html {
    log!(add(1, 2));
    html! {
        <main class="p-5 text-4xl">
        <h1>{"Welcome Yew + YewUI"}</h1>
        <dl>
            <dt class="pt-4 text-2xl">{"基础按钮"}</dt>
            <dd class="pb-6">
                <Button text="Primary 按钮" custom_class="mr-2" click={|_| log!("点击了确认按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Warning 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Danger 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Danger}/>
                <Button text="Success 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Success}/>
                <Button text="Info 按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Info}/>
                <Button text="默认按钮" custom_class="mr-2" click={|_| log!("点击了取消按钮")} />
            </dd>
            <dt class="py-4 text-2xl">{"圆角按钮"}</dt>
            <dd class="pb-6">
                <Button text="Full" rounded={Rounded::Full} custom_class="mb-2 mr-2" click={|_| log!("点击了确认按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Xl3" rounded={Rounded::Xl3} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Xl2" rounded={Rounded::Xl2} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Xl" rounded={Rounded::Xl} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Lg"  rounded={Rounded::Lg} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Rounded"  rounded={Rounded::Rounded} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Sm"  rounded={Rounded::Sm} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="None"  rounded={Rounded::None} custom_class="mb-2 mr-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
            </dd>
            <dt class="py-4 text-2xl">{"块按钮"}</dt>
            <dd class="pb-6">
                <Button text="Primary 按钮" block=true custom_class="mb-2" click={|_| log!("点击了确认按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Warning 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Warning}/>
                <Button text="Danger 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Danger}/>
                <Button text="Success 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Success}/>
                <Button text="Info 按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} r#type={TypeStyle::Info}/>
                <Button text="默认按钮" block=true custom_class="mb-2" click={|_| log!("点击了取消按钮")} />
            </dd>
            <dt class="py-4 text-2xl">{"按钮尺寸"}</dt>
            <dd class="pb-6">
                <Button text="Xs" size={SizeStyle::Xs} custom_class="mr-2" click={|_| log!("点击了 Xs 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Sm" size={SizeStyle::Sm} custom_class="mr-2" click={|_| log!("点击了 Sm 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Default" custom_class="mr-2" click={|_| log!("点击了 Xs 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Base" size={SizeStyle::Base} custom_class="mr-2" click={|_| log!("点击了 Lg 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Lg" size={SizeStyle::Lg} custom_class="mr-2" click={|_| log!("点击了 Lg 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl" size={SizeStyle::Xl} custom_class="mr-2" click={|_| log!("点击了 Xl 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl2" size={SizeStyle::Xl2} custom_class="mr-2" click={|_| log!("点击了 Xl2 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl3" size={SizeStyle::Xl3} custom_class="mr-2" click={|_| log!("点击了 Xl3 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl4" size={SizeStyle::Xl4} custom_class="mr-2" click={|_| log!("点击了 Xl4 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl5" size={SizeStyle::Xl5} custom_class="mr-2" click={|_| log!("点击了 Xl5 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl6" size={SizeStyle::Xl6} custom_class="mr-2" click={|_| log!("点击了 Xl6 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl7" size={SizeStyle::Xl7} custom_class="mr-2" click={|_| log!("点击了 Xl7 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl8" size={SizeStyle::Xl8} custom_class="mr-2" click={|_| log!("点击了 Xl8 按钮")} r#type={TypeStyle::Primary}/>
                <Button text="Xl9" size={SizeStyle::Xl9} custom_class="mr-2" click={|_| log!("点击了 Xl9 按钮")} r#type={TypeStyle::Primary}/>
            </dd>
        </dl>
    </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

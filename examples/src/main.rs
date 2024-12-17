use yew::prelude::*;
use yew_ui::components::prelude::*;
use yew_ui::components::svg::IconName;
use yew_ui::gloo::console::log;
use yew_ui::gloo::dialogs::alert;

use yew_ui::toolkit::math::add;

#[function_component(App)]
fn app() -> Html {
    log!(add(1, 2));
    html! {
        <main class="p-5 text-4xl">
        <h1>{"YewUI = Yew + Tailwindcss"}</h1>
        <dl>
            <dt class="pt-4 text-2xl">{"Basic usage"}</dt>
            <dd class="pb-6">
                <Button text="Primary" custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" custom_class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" custom_class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" custom_class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" custom_class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" custom_class="mr-2" />
            </dd>
            <dt class="pt-4 text-2xl">{"Disabled Button"}</dt>
            <dd class="pb-6">
                <Button text="Primary" disabled={true}  custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" disabled={true} custom_class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" disabled={true} custom_class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" disabled={true} custom_class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" disabled={true} custom_class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" disabled={true} custom_class="mr-2" />
            </dd>
            <dt class="pt-4 text-2xl">{"Basic link button"}</dt>
            <dd class="pb-6">
                <Button text="Primary" link={true} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" link={true} custom_class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" link={true} custom_class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" link={true} custom_class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" link={true} custom_class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" link={true} custom_class="mr-2" />
            </dd>
            <dt class="pt-4 text-2xl">{"Disabled link button"}</dt>
            <dd class="pb-6">
                <Button text="Primary" disabled={true} link={true} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" disabled={true} link={true} custom_class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" disabled={true} link={true} custom_class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" disabled={true} link={true} custom_class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" disabled={true} link={true} custom_class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" disabled={true} link={true} custom_class="mr-2" />
            </dd>
            <dt class="py-4 text-2xl">{"Rounded"}</dt>
            <dd class="pb-6">
                <Button text="Full" rounded={Rounded::Full} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Xl3" rounded={Rounded::Xl3} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Xl2" rounded={Rounded::Xl2} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Xl" rounded={Rounded::Xl} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Lg"  rounded={Rounded::Lg} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Rounded"  rounded={Rounded::Rounded} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Sm"  rounded={Rounded::Sm} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="None"  rounded={Rounded::None} custom_class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
            </dd>
            <dt class="py-4 text-2xl">{"Icon Button"}</dt>
            <dd class="pb-6">
                { "developing ... "}
            </dd>
            <dt class="py-4 text-2xl">{"Block"}</dt>
            <dd class="pb-6 w-[700px]">
                <Button text="Primary" block=true custom_class="mb-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" block=true custom_class="mb-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" block=true custom_class="mb-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" block=true custom_class="mb-2" r#type={TypeStyle::Success}/>
                <Button text="Info" block=true custom_class="mb-2" r#type={TypeStyle::Info}/>
                <Button text="Default" block=true custom_class="mb-2" />
            </dd>
            <dt class="py-4 text-2xl">{"Sizes"}</dt>
            <dd class="pb-6">
                <Button text="Xs" size={SizeStyle::Xs} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Sm" size={SizeStyle::Sm} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Default" custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Base" size={SizeStyle::Base} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Lg" size={SizeStyle::Lg} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Xl" size={SizeStyle::Xl} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl2" size={SizeStyle::Xl2} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl3" size={SizeStyle::Xl3} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl4" size={SizeStyle::Xl4} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl5" size={SizeStyle::Xl5} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl6" size={SizeStyle::Xl6} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl7" size={SizeStyle::Xl7} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl8" size={SizeStyle::Xl8} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl9" size={SizeStyle::Xl9} custom_class="mr-2" r#type={TypeStyle::Primary}/>
            </dd>
            <dt class="py-4 text-2xl">{"Event"}</dt>
            <dd class="pb-6">
                <Button text="console" custom_class="mb-2 mr-2" click={|_| log!("The button has been clicked")} r#type={TypeStyle::Success}/>
                <Button text="alert" custom_class="mb-2" click={|_| alert("The button has been clicked")} r#type={TypeStyle::Success}/>
            </dd>

            <dl class="mt-4">
                <dt class="py-4 text-2xl">{"Icon Button"}</dt>
                <dd class="pb-6">
                    <Button rounded={Rounded::Full} icon={IconName::Home} size={SizeStyle::Xs} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                    <Button text="Xs" icon={IconName::Home} size={SizeStyle::Xs} custom_class="mr-2" r#type={TypeStyle::Primary}/>
                    <Button text="Xs" icon={IconName::Home} size={SizeStyle::Xs} custom_class="mr-2" r#type={TypeStyle::Danger}/>
                    <Button text="custom_class" rounded={Rounded::Full} icon={IconName::Home} size={SizeStyle::Lg} custom_class="mr-2 px-6" r#type={TypeStyle::Danger}/>
                </dd>
            </dl>
        </dl>
        <hr />
        <br />
        <hr />
        <dl class="mt-4">
            <dt class="py-4 text-2xl">{"Icon Component"}</dt>
            <dd class="pb-6">
                <Icon name={IconName::Home} color={Colors::Red(Levels::N500)}/>
            </dd>
        </dl>
    </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

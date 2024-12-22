use yew::prelude::*;
use yew_ui::components::prelude::*;
use yew_ui::components::svg::IconName;
use yew_ui::gloo::console::log;
use yew_ui::gloo::dialogs::alert;

use yew_ui::toolkit::math::add;

use crate::components::Footer;

#[function_component(PageButton)]
pub fn page_button() -> Html {
    log!(add(1, 2));
    html! {
        <main class="text-4xl">
        <dl>
            <dt class="pt-4 text-2xl">{"Basic usage"}</dt>
            <dd class="pb-6">
                <Button text="Primary" class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" class="mr-2" />
            </dd>
            <dt class="pt-4 text-2xl">{"Disabled Button"}</dt>
            <dd class="pb-6">
                <Button text="Primary" disabled={true}  class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" disabled={true} class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" disabled={true} class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" disabled={true} class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" disabled={true} class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" disabled={true} class="mr-2" />
            </dd>
            <dt class="pt-4 text-2xl">{"Basic link button"}</dt>
            <dd class="pb-6">
                <Button text="Primary" link={true} class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" link={true} class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" link={true} class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" link={true} class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" link={true} class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" link={true} class="mr-2" />
            </dd>
            <dt class="pt-4 text-2xl">{"Disabled link button"}</dt>
            <dd class="pb-6">
                <Button text="Primary" disabled={true} link={true} class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" disabled={true} link={true} class="mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" disabled={true} link={true} class="mr-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" disabled={true} link={true} class="mr-2" r#type={TypeStyle::Success}/>
                <Button text="Info" disabled={true} link={true} class="mr-2"  r#type={TypeStyle::Info}/>
                <Button text="Default" disabled={true} link={true} class="mr-2" />
            </dd>
            <dt class="py-4 text-2xl">{"Rounded"}</dt>
            <dd class="pb-6">
                <Button text="Full" rounded={Rounded::Full} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Xl3" rounded={Rounded::Xl3} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Xl2" rounded={Rounded::Xl2} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Xl" rounded={Rounded::Xl} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Lg"  rounded={Rounded::Lg} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Rounded"  rounded={Rounded::Rounded} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="Sm"  rounded={Rounded::Sm} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
                <Button text="None"  rounded={Rounded::None} class="mb-2 mr-2" r#type={TypeStyle::Warning}/>
            </dd>
            <dt class="py-4 text-2xl">{"Icon Button"}</dt>
            <dd class="pb-6">
                { "developing ... "}
            </dd>
            <dt class="py-4 text-2xl">{"Block"}</dt>
            <dd class="pb-6 w-[700px]">
                <Button text="Primary" block=true class="mb-2" r#type={TypeStyle::Primary}/>
                <Button text="Warning" block=true class="mb-2" r#type={TypeStyle::Warning}/>
                <Button text="Danger" block=true class="mb-2" r#type={TypeStyle::Danger}/>
                <Button text="Success" block=true class="mb-2" r#type={TypeStyle::Success}/>
                <Button text="Info" block=true class="mb-2" r#type={TypeStyle::Info}/>
                <Button text="Default" block=true class="mb-2" />
            </dd>
            <dt class="py-4 text-2xl">{"Sizes"}</dt>
            <dd class="pb-6">
                <Button text="Xs" size={SizeStyle::Xs} class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Sm" size={SizeStyle::Sm} class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Default" class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Base" size={SizeStyle::Base} class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Lg" size={SizeStyle::Lg} class="mr-2" r#type={TypeStyle::Primary}/>
                <Button text="Xl" size={SizeStyle::Xl} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl2" size={SizeStyle::Xl2} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl3" size={SizeStyle::Xl3} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl4" size={SizeStyle::Xl4} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl5" size={SizeStyle::Xl5} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl6" size={SizeStyle::Xl6} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl7" size={SizeStyle::Xl7} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl8" size={SizeStyle::Xl8} class="mr-2" r#type={TypeStyle::Primary}/>
                // <Button text="Xl9" size={SizeStyle::Xl9} class="mr-2" r#type={TypeStyle::Primary}/>
            </dd>
            <dt class="py-4 text-2xl">{"Event"}</dt>
            <dd class="pb-6">
                <Button text="console" class="mb-2 mr-2" click={|_| log!("The button has been clicked")} r#type={TypeStyle::Success}/>
                <Button text="alert" class="mb-2" click={|_| alert("The button has been clicked")} r#type={TypeStyle::Success}/>
            </dd>

            <dl class="mt-4">
                <dt class="py-4 text-2xl">{"Icon Button"}</dt>
                <dd class="pb-6">
                    <Button rounded={Rounded::Full} icon={IconName::Home} size={SizeStyle::Xs} class="mr-2" r#type={TypeStyle::Primary}/>
                    <Button text="Xs" icon={IconName::Home} size={SizeStyle::Xs} class="mr-2" r#type={TypeStyle::Primary}/>
                    <Button text="Xs" icon={IconName::Home} size={SizeStyle::Xs} class="mr-2" r#type={TypeStyle::Danger}/>
                    <Button text="class" rounded={Rounded::Full} icon={IconName::Home} size={SizeStyle::Lg} class="mr-2 px-6" r#type={TypeStyle::Danger}/>
                </dd>
            </dl>
        </dl>
        <Footer prev={"/"} next={"/icon"} />
    </main>
    }
}

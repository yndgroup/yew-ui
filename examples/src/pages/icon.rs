use yew::prelude::*;
use yew_ui::components::prelude::*;
use yew_ui::components::svg::IconName;
use yew_ui::gloo::console::log;

use yew_ui::toolkit::math::add;

use crate::components::Footer;

#[function_component(PageIcon)]
pub fn page_icon() -> Html {
    log!(add(1, 2));
    html! {
       <>
            <dl class="mt-4">
            <dd class="pb-6">
                <Icon name={IconName::Home} color={Colors::Red(Levels::N500)}/>
            </dd>
            </dl>
            <Footer prev={"/docs"} next={"/docs"} />
        </>
    }
}

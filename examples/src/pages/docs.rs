use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::Footer, Route};

#[function_component(PageDocs)]
pub fn page_docs() -> Html {
    html! {
        <div class="home">
            <dl>
                <dt class="border-b-2 border-dashed text-4xl pb-2">{"Basic"}</dt>
                <dd>
                    <ul>
                        <li class="font-bold pt-3 text-blue-600">
                            <Link<Route> to={Route::PageButton}>{ "Button" }</Link<Route>>
                        </li>
                        <li class="font-bold pt-3 text-blue-600">
                            <Link<Route> to={Route::PageIcon}>{ "Icon" }</Link<Route>>
                        </li>
                    </ul>
                </dd>
            </dl>
            <Footer prev={"/docs"} next={"/icon"} />
        </div>
    }
}

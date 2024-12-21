use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Switch;

use crate::components::*;

use crate::routes::switch;
use crate::routes::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <div class="p-4">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

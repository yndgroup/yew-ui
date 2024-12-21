use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{PageButton, PageDocs, PageHome, PageIcon};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/button")]
    PageButton,

    #[at("/icon")]
    PageIcon,

    #[at("/docs")]
    PageDocs,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {  <PageHome /> },
        Route::PageButton => html! {
            <PageButton />
        },
        Route::PageIcon => html! {
            <PageIcon />
        },
        Route::PageDocs => html! {
            <PageDocs />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
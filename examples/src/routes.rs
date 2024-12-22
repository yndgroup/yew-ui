use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{PageBorder, PageButton, PageContainer, PageDocs, PageHome, PageIcon};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/button")]
    PageButton,

    #[at("/border")]
    PageBorder,

    #[at("/icon")]
    PageIcon,

    #[at("/docs")]
    PageDocs,

    #[at("/container")]
    PageContainer,

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
        Route::PageBorder => html! {
            <PageBorder />
        },
        Route::PageDocs => html! {
            <PageDocs />
        },
        Route::PageContainer => html! {
            <PageContainer />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
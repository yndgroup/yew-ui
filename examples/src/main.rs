use pages::App;
use routes:: Route;

mod pages;
mod routes;
mod components;

fn main() {
    yew::Renderer::<App>::new().render();
}

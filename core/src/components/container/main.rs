use yew::prelude::*;

pub struct Main {
    pub props: MainProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct MainProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: String,
}

pub enum MainMsg {
}

impl Component for Main {
    type Message = MainMsg;
    type Properties = MainProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main class={self.props.class.clone()}>
            {self.props.children.clone()}
            </main>
        }
    }
}
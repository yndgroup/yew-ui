use yew::prelude::*;

pub struct Header {
    pub props: HeaderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: String,
}

pub enum HeaderMsg {
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = HeaderProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header class={self.props.class.clone()}>
            {self.props.children.clone()}
            </header>
        }
    }
}
use yew::prelude::*;

pub struct Footer {
    pub props: FooterProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: String,
}

pub enum FooterMsg {
}

impl Component for Footer {
    type Message = FooterMsg;
    type Properties = FooterProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class={self.props.class.clone()}>
            {self.props.children.clone()}
            </footer>
        }
    }
}
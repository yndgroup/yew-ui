use yew::prelude::*;

pub struct Aside {
    pub props: AsideProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct AsideProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub width: String,
}

pub enum AsideMsg {
}

impl Component for Aside {
    type Message = AsideMsg;
    type Properties = AsideProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut class: String = String::new();
        if !ctx.props().class.is_empty() {
            class = ctx.props().class.clone();
        }
        if !ctx.props().width.is_empty() {
            class = format!("{} w-{}", class , ctx.props().width);
        }

        html! {
            <aside class={class}>
            {self.props.children.clone()}
            </aside>
        }
    }
}
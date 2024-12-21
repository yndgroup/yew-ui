use yew::prelude::*;

pub struct Tooltip {
    pub props: TooltipProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct TooltipProps {
    #[prop_or_default]
    pub children: Children
}

pub enum TooltipMsg {
}

impl Component for Tooltip {
    type Message = TooltipMsg;
    type Properties = TooltipProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            {self.props.children.clone()}
            </>
        }
    }
}
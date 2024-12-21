use yew::prelude::*;

pub struct Alert {
    pub props: AlertProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct AlertProps {

    #[prop_or_default]
    pub children: Children
    
}

pub enum AlertMsg {
}

impl Component for Alert {
    type Message = AlertMsg;
    type Properties = AlertProps;

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
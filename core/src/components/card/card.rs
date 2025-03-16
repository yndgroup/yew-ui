use yew::prelude::*;

#[derive(Debug)]
pub struct Card {
    pub props: CardProps
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub border: String,
}

pub enum CardMsg {
}

impl Component for Card {
    type Message = CardMsg;
    type Properties = CardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        println!("card, {:?}", ctx);
        let mut class =  String::from("");

        if !self.props.class.is_empty() {
            class = format!("{} {}", class, self.props.class.clone());
        }

        if !self.props.border.is_empty() {
            class = format!("{} {}", class, self.props.border.clone());
        }

        html! {
            <div class={class}>
            {"1"}
            {self.props.children.clone()}
            </div>
        }
    }
}
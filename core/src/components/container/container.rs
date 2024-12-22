use gloo::console::log;
use yew::prelude::*;

use crate::components::theme::Border;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Column
    }
}

#[derive(Debug)]
pub struct Container {
    pub props: ContainerProps
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub direction: Direction,

    #[prop_or_default]
    pub border: String,
}

pub enum ContainerMsg {
}

impl Component for Container {
    type Message = ContainerMsg;
    type Properties = ContainerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        print!("container, {:?}", ctx);
        let mut class =  String::from("");

        if !self.props.class.is_empty() {
            class = format!("{} {}", class, self.props.class.clone());
        }

        if !self.props.border.is_empty() {
            class = format!("{} {}", class, self.props.border.clone());
        }

        html! {
            <div class={class}>
            {self.props.children.clone()}
            </div>
        }
    }
}
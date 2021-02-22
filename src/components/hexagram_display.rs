use std::mem;
use crate::components::{Hexagram, Display};
use yew::prelude::*;

pub struct HexagramDisplay {
    props: Props,
}

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub hex: Hexagram,

    #[prop_or_default]
    pub id: String,
}

impl Component for HexagramDisplay {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
        mem::swap(&mut self.props, &mut props);
        self.props != props
    }

    fn view(&self) -> Html {
        let hex = &self.props.hex;
        html! {
            <Display id={ self.props.id.clone() }>
                <div class="hexagram_display">
                    { hex.render() }

                    <div class="hexagram_name">
                    { format!("{} {}", hex.id(), hex.name()) }
                    </div>
                </div>
            </Display>
        }
    }
}

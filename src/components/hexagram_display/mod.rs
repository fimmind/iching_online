mod line;

use crate::types::{Hexagram, LineType};
use line::Line;
use yew::prelude::*;

pub struct HexagramDisplay {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetLine(usize, LineType),
    SwitchLine(usize),
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub hexagram: Hexagram,
}

impl Component for HexagramDisplay {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetLine(i, line_type) => self.props.hexagram.set_line(i, line_type),
            Msg::SwitchLine(i) => {
                self.props.hexagram.switch_line(i);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let render_line = |l| html! { <li><Line line_type=l/></li> };
        let hexagram = &self.props.hexagram;
        html! {
            <div class="hexagram">
                <ul class="hexagram_lines">
                    {for hexagram.lines().map(render_line)}
                </ul>

                <div class="hexagram_name">{ format!("{} {}", hexagram.id(), hexagram.name()) }</div>
            </div>
        }
    }
}

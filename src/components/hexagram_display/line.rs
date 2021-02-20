use yew::prelude::*;
use crate::types::LineType;

pub struct Line {
    props: Props,
}

#[derive(Debug, Clone, Properties, Eq, PartialEq)]
pub struct Props {
    pub line_type: LineType,
}

impl Component for Line {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let middle = match self.props.line_type {
            LineType::Yang => "filled",
            LineType::Yin => "empty",
        };
        html! {
            <div class="hexagram_line">
                <div class="filled"></div>
                <div class=middle></div>
                <div class="filled"></div>
            </div>
        }
    }
}

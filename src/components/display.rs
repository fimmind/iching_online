use yew::prelude::*;

pub struct Display {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,
}

impl Component for Display {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="display" id=self.props.id.clone()>
                { self.props.children.clone() }
            </div>
        }
    }
}

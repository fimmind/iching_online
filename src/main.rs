#![allow(dead_code)]

mod components;

use components::{Display, Hexagram, HexagramGenerator, HexagramDisplay};
use yew::prelude::*;

struct Model {
    present_hexagram: Hexagram,
    future_hexagram: Hexagram,
    link: ComponentLink<Self>,
}

enum Msg {
    SetPresentHexagram(Hexagram),
    SetFutureHexagram(Hexagram),
    SetHexagrams(Hexagram, Hexagram),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            present_hexagram: Hexagram::new(),
            future_hexagram: Hexagram::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPresentHexagram(hex) => {
                self.present_hexagram = hex;
                true
            }
            Msg::SetFutureHexagram(hex) => {
                self.future_hexagram = hex;
                true
            }
            Msg::SetHexagrams(present_hex, future_hex) => {
                self.update(Msg::SetPresentHexagram(present_hex));
                self.update(Msg::SetFutureHexagram(future_hex));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="content">
                <HexagramGenerator
                 oninput=self.link.callback(|(phex, fhex)| Msg::SetHexagrams(phex, fhex)),
                 id="hexagram_generator"/>
                <HexagramDisplay
                 hex={ self.present_hexagram.clone() },
                 id="future_hexagram"/>
                <HexagramDisplay
                 hex={ self.future_hexagram.clone() },
                 id="future_hexagram"/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>()
}

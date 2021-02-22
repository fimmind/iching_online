#![allow(dead_code)]
#![recursion_limit = "640"]

mod components;

use components::{Display, Hexagram, HexagramDisplay, HexagramGenerator};
use yew::prelude::*;

struct Model {
    present_hexagram: Hexagram,
    future_hexagram: Hexagram,
    info_hexagram: HexagramType,
    link: ComponentLink<Self>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum HexagramType {
    Present,
    Future,
}

enum Msg {
    SetPresentHexagram(Hexagram),
    SetFutureHexagram(Hexagram),
    SetHexagrams(Hexagram, Hexagram),
    SetInfoHexagram(HexagramType),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            present_hexagram: Hexagram::new(),
            future_hexagram: Hexagram::new(),
            info_hexagram: HexagramType::Present,
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
            Msg::SetInfoHexagram(hex) => {
                if self.info_hexagram != hex {
                    self.info_hexagram = hex;
                    return true
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let info_hex = match self.info_hexagram {
            HexagramType::Present => &self.present_hexagram,
            HexagramType::Future => &self.future_hexagram,
        };
        html! {
            <div id="content">
                <HexagramGenerator
                 oninput=self.link.callback(|(phex, fhex)| Msg::SetHexagrams(phex, fhex)),
                 id="hexagram_generator"/>
                <div onclick=self.link.callback(|_| Msg::SetInfoHexagram(HexagramType::Present))>
                    <HexagramDisplay
                    hex={ self.present_hexagram.clone() },
                    id="present_hexagram"/>
                </div>
                <div onclick=self.link.callback(|_| Msg::SetInfoHexagram(HexagramType::Future))>
                    <HexagramDisplay
                    hex={ self.future_hexagram.clone() },
                    id="future_hexagram"/>
                </div>

                <Display id="hexagram_info">
                    { info_hex.description() }
                </Display>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>()
}

#![allow(dead_code)]
#![recursion_limit = "640"]

mod components;

use std::mem;

use components::{Display, Hexagram, HexagramDisplay, HexagramGenerator};
use enum_map::{enum_map, Enum, EnumMap};
use yew::prelude::*;

struct Model {
    hexagrams: EnumMap<HexagramType, Hexagram>,
    info_hexagram: HexagramType,
    link: ComponentLink<Self>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
enum HexagramType {
    Present,
    Future,
}

enum Msg {
    SetHexagram(HexagramType, Hexagram),
    SetHexagrams(Hexagram, Hexagram),
    SetInfoHexagram(HexagramType),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            hexagrams: enum_map! {
                HexagramType::Present => Hexagram::new(),
                HexagramType::Future => Hexagram::new(),
            },
            info_hexagram: HexagramType::Present,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetHexagram(hex_type, hex) => {
                self.hexagrams[hex_type] = hex;
                true
            }
            Msg::SetHexagrams(present_hex, future_hex) => {
                self.update(Msg::SetHexagram(HexagramType::Present, present_hex));
                self.update(Msg::SetHexagram(HexagramType::Future, future_hex));
                true
            }
            Msg::SetInfoHexagram(hex) => {
                if self.info_hexagram != hex {
                    self.info_hexagram = hex;
                    return true;
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (mut present_hex_class, mut future_hex_class) = ("selected_hexagram", "");
        if let HexagramType::Future = self.info_hexagram {
            mem::swap(&mut present_hex_class, &mut future_hex_class);
        }

        html! {
            <div id="content">
                <div id="hexagram_input">
                    <HexagramGenerator
                        oninput=self.link.callback(|(phex, fhex)| Msg::SetHexagrams(phex, fhex)),
                        id="hexagram_generator"/>
                    <div onclick=self.link.callback(|_| Msg::SetInfoHexagram(HexagramType::Present))
                        class={ present_hex_class }>
                        <HexagramDisplay
                            hex={ self.hexagrams[HexagramType::Present].clone() },
                            id="present_hexagram"/>
                    </div>
                    <div onclick=self.link.callback(|_| Msg::SetInfoHexagram(HexagramType::Future))
                        class={ future_hex_class }>
                        <HexagramDisplay
                            hex={ self.hexagrams[HexagramType::Future].clone() },
                            id="future_hexagram"/>
                    </div>
                </div>

                <Display id="hexagram_info">
                    { self.hexagrams[self.info_hexagram].description() }
                </Display>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>()
}

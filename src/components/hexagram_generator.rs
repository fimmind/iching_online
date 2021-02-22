use rand::prelude::*;
use std::iter;
use yew::prelude::*;

use crate::components::hexagram::{Hexagram, Line};

pub struct HexagramGenerator {
    coin_drops: Vec<bool>,
    link: ComponentLink<Self>,
    props: Props,
    rng: ThreadRng,
}

impl HexagramGenerator {
    fn generate(&mut self) -> (Hexagram, Hexagram) {
        self.coin_drops = iter::repeat_with(|| self.rng.gen()).take(6 * 3).collect();
        self.coin_drops.reverse();

        (self.get_present_hexagram(), self.get_future_hexagram())
    }

    fn get_present_hexagram(&self) -> Hexagram {
        let lines = self.coin_drops.chunks_exact(3).map(|drops| {
            match drops.iter().filter(|&&b| b).count() {
                1 | 3 => Line::Yang,
                0 | 2 => Line::Yin,
                _ => unreachable!(),
            }
        });
        Hexagram::from_lines(lines)
    }

    fn get_future_hexagram(&self) -> Hexagram {
        let lines = self.coin_drops.chunks_exact(3).map(|drops| {
            match drops.iter().filter(|&&b| b).count() {
                0 | 1 => Line::Yang,
                2 | 3 => Line::Yin,
                _ => unreachable!(),
            }
        });
        Hexagram::from_lines(lines)
    }

    fn default_coin_drops() -> Vec<bool> {
        iter::repeat(&[true, false, false])
            .take(6)
            .flatten()
            .copied()
            .collect()
    }
}

pub enum Msg {
    Generate,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub oninput: Callback<(Hexagram, Hexagram)>,

    #[prop_or_default]
    pub id: String,
}

impl Component for HexagramGenerator {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            coin_drops: Self::default_coin_drops(),
            rng: thread_rng(),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Generate => {
                let hexs = self.generate();
                self.props.oninput.emit(hexs);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        let render_cell = |&b: &bool| html! {<td>{ if b { 1 } else { 0 } }</td>};
        let render_row = |row: &[bool]| html! {<tr>{for row.iter().map(render_cell)}</tr>};
        let rows = self.coin_drops.chunks(3).map(render_row);
        html! {
            <div
             onclick=self.link.callback(|_| Msg::Generate)
             class="hexagram_generator"
             id={ self.props.id.clone() }>
                <table>{for rows}</table>
            </div>
        }
    }
}

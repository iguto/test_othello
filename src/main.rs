#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::console::ConsoleService;

mod map;

use map::{Cell, CellColors, Coordinate, Map};

struct Context {
    console: ConsoleService,
}

struct Model {
    map: Map,
    player: CellColors,
}

#[derive(Debug)]
enum Msg {
    Nope,
    Hand(usize, usize),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            map: Map::new(8, 8),
            player: CellColors::Black,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        println!("msg: {:?}", msg);
        match msg {
            Msg::Hand(row, column) => {
                self.map.put_hand(row, column, self.player);
                println!("in hand row:{} column:{}", row, column);
                self.switch_player();
            }
            Msg::Nope => (),
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                { self.render_player_indicator() }
                { self.render_map() }
            </div>
        }
    }
}

impl Model {
    fn render_map(&self) -> Html<Context, Self> {
        let render_map_elem = |cell: &Cell| {
            let c = cell.clone();
            match cell.color {
                CellColors::Empty
                    if self.map
                        .is_reversible(Coordinate(cell.row, cell.column), self.player) =>
                {
                    html!{
                        <td class=("gray-cell", "clickable"), onclick=move |_: MouseData| Msg::Hand(c.row, c.column), />
                    }
                }
                CellColors::Empty => html!{ <td class="gray-cell", ></td> },
                CellColors::Black => html!{ <td class="black-cell" ,></td> },
                CellColors::White => html!{ <td class="white-cell" ,></td> },
            }
        };

        html!{
            <table>{ for self.map.inner_map.iter().map(|column| {
                html!{
                    <tr>
                        { for column.iter().map(|cell| render_map_elem(cell)) }
                    </tr>
                }
            })}</table>
        }
    }

    fn render_player_indicator(&self) -> Html<Context, Self> {
        html! {
            <div class="player-indicator-container", >
                <span>{ "player:" }</span>
                <span class=("player-indicator", {
                    match self.player {
                        CellColors::Black => "player-black",
                        CellColors::White => "player-white",
                        _ => "",
                    }
                }),></span>
            </div>
        }
        // match self.player {
        //     CellColors::Black => html!{
        //         <p>player: <div class=("player-indicator", "player-black"),></div></p>
        //     },
        //     CellColors::White => html!{
        //         <p>player: <div class=("player-indicator", "player-white"),></div></p>
        //     },
        //     _ => unreachable!(),
        // }
    }

    fn switch_player(&mut self) {
        self.player = match self.player {
            CellColors::White => CellColors::Black,
            CellColors::Black => CellColors::White,
            CellColors::Empty => unreachable!(),
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

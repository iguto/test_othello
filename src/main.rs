extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;

mod map;
 
use map::{Cell, CellColors, Map, Coordinate};

struct Context {
    console: ConsoleService,
}

struct Model {
    list: Vec<String>,
    text_value: String,
    map: Map,
    player: CellColors,
}

#[derive(Debug)]
enum Msg {
    Add,
    EditInput(String),
    Nope,
    Hand(usize, usize),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            list: vec![],
            text_value: String::new(),
            map: Map::new(8, 8),
            player: CellColors::Black,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        println!("msg: {:?}", msg);
        match msg {
            Msg::Add => {
                if self.text_value.is_empty() {
                    return false;
                }
                let text = self.text_value.clone();
                self.list.push(text);
                self.text_value.clear();
            }
            Msg::EditInput(text) => {
                self.text_value = text;
            },
            Msg::Hand(row, column) => {
                self.map.put_hand(row, column, self.player);
                println!("in hand row:{} column:{}", row, column);
                self.switch_player();
            },
            Msg::Nope => (),
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <p>{ Local::now() }</p>
                <hr />
                { self.text_input() }
                { self.render_list() }
                <hr />
                { self.render_map() }
            </div>
        }
    }
}

impl Model {
    fn text_input(&self) -> Html<Context, Self> {
        html! {
            <input type="text",
                   value=&self.text_value,
                   oninput=|e: InputData| Msg::EditInput(e.value),
                   onkeypress=move |e: KeyData| {
                       if e.key == "Enter" { Msg::Add } else { Msg::Nope }
                   },
            />
        }
    }
    fn render_list(&self) -> Html<Context, Self> {
        html!{
            <ul>
                { for self.list.iter().enumerate().map(view_list_elem) }
            </ul>
        }
    }

    fn render_map(&self) -> Html<Context, Self> {
        let render_map_elem = |cell: &Cell| { 
            let c = cell.clone();
            match cell.color { 
                CellColors::Empty if self.is_reversible(Coordinate(cell.row, cell.column), self.player) => { 
                    html!{
                        <td class=("gray-cell", "clickable"),
                            onclick=move |_: MouseData| Msg::Hand(c.row, c.column),
                        />
                    }
                },
                CellColors::Empty => html!{ <td class="gray-cell", ></td> },
                CellColors::Black => html!{ <td class="black-cell" ,></td> },
                CellColors::White => html!{ <td class="white-cell" ,></td> },
            }
        };

        html!{
            <table>
                { for self.map.inner_map.iter().map(|column| {
                    html!{
                        <tr>
                            { for column.iter().map(|cell| render_map_elem(cell)) }
                        </tr>
                    }
                  }) 
                }
            </table>
        }
    }

    fn is_reversible(&self, cursor: Coordinate, player: CellColors) -> bool {
        // self.map.inner_map[row][column]
        let dirs: [(i64, i64); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
        for dir in dirs.iter() {
            if self.is_reversible_dir(cursor, player, *dir) {
                return true
            }
        }
        false
    }

    fn is_reversible_dir(&self, cursor: Coordinate, player: CellColors, dir: (i64, i64)) -> bool {
        let mut cursor = cursor.clone();
        let mut reversible_count = 0;

        loop {
            cursor = match cursor.next(dir) {
                Some(coord) => coord,
                None => return false,
            };

            let cell_color = self.map.inner_map[cursor.0][cursor.1].color;
            match player {
                CellColors::Black => { 
                    match cell_color {
                        CellColors::White => reversible_count += 1,
                        CellColors::Black if reversible_count > 0 => return true,
                        _ => return false,
                    }
                },
                CellColors::White => match cell_color {
                    CellColors::Black => reversible_count += 1,
                    CellColors::White if reversible_count > 0 => return true,
                    _ => return false,
                },
                _ => return false,
            };
        }
    }

    fn switch_player(&mut self) {
        self.player = match self.player {
            CellColors::White => CellColors::Black,
            CellColors::Black => CellColors::White,
            CellColors::Empty => unreachable!(),
        }
    }
}

fn view_list_elem((_, elem): (usize, &String)) -> Html<Context, Model> {
    html!{
        <li>{ elem }</li>
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

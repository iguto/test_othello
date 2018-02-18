extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

struct Model {
    list: Vec<String>,
    text_value: String,
}

#[derive(Debug)]
enum Msg {
    Add,
    EditInput(String),
    Nope,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            list: vec![],
            text_value: String::new(),
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
                <p>{ Local::now() }</p>
                <hr />
                { self.text_input() }
                { self.render_list() }
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
}

fn view_list_elem((index, elem): (usize, &String)) -> Html<Context, Model> {
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

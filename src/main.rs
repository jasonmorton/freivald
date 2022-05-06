use yew::prelude::*;

use ark_bls12_381::Fr as F;
use nalgebra::DMatrix;
mod freivald;

mod fetcher;

use wasm_bindgen_futures::spawn_local;
use web_sys::console;

async fn my_async_fn() -> String {
    String::from("Hello from pretend server")
}

enum Msg {
    MultiplyMatricesLocally,
    MultiplyMatricesServer,
    Reset,
    Done(u64),
}

struct Model {
    a: DMatrix<F>,
    b: DMatrix<F>,
    c: Option<DMatrix<F>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (a, b) = freivald::generate_instance(2, 3, 4);
        Self {
            a: a,
            b: b,
            c: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MultiplyMatricesLocally => {
                self.c = Some(&self.a * &self.b);
                true // rerender
            }
            Msg::MultiplyMatricesServer => {
                // link = ctx.link() might also work with move
                let cb = ctx.link().callback(|num: u64| Msg::Done(num));
                spawn_local(async move {
                    let bar = my_async_fn().await;
                    let baz = fetcher::fetch_reqwasm().await;
                    console::log_1(&baz.into());
                    cb.emit(9);
                });

                //                self.c = Some(&self.a * &self.b);
                false // rerender
            }
            Msg::Reset => {
                let (a, b) = freivald::generate_instance(2, 3, 4);
                self.a = a;
                self.b = b;
                self.c = None;
                true
            }
            Msg::Done(c) => {
                // set self.c to c
                self.c = Some(&self.a * &self.b);
                console::log_1(&"Done".into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
        <div>
        <button onclick={link.callback(|_| Msg::MultiplyMatricesLocally)}>{ "Multiply in browser and verify in browser" }</button>
        <button onclick={link.callback(|_| Msg::MultiplyMatricesServer)}>{ "Multiply on server and verify in browser" }</button>
        <button onclick={link.callback(|_| Msg::Reset)}>{ "Reset A and B" }</button>
        <p>{"A:"}{ &self.a }</p>
        <p>{"B:"}{ &self.b }</p>
        <p>{"C:"}{ if let Some(c) = &self.c {c.to_string()} else {"None".to_string()} }</p>
        <p>{freivald::perhapsverify(self.a.clone(),self.b.clone(),self.c.clone())}</p>
        </div>



        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

fn main() {
    yew::start_app::<Model>();
    // yew::start_app_with_props::<Model>(..).
}

use yew::prelude::*;

use ark_bls12_381::Fr as F;
use nalgebra::DMatrix;
mod freivald;

enum Msg {
    MultiplyMatrices,
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MultiplyMatrices => {
                //                self.a *= F::from(2u64);
                self.c = Some(&self.a * &self.b);
                true // rerender
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
        <div>
        <button onclick={link.callback(|_| Msg::MultiplyMatrices)}>{ "Compute and verify" }</button>
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

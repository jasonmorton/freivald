use yew::prelude::*;

use ark_bls12_381::Fr;
mod freivald;

enum Msg {
    AddOne,
}

struct Model {
    value: Fr,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: Fr::from(1u64),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value *= Fr::from(2u64);
                true // rerender
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
		<p>{ self.value }</p>
		<p>{freivald::test()}</p>
		</div>



        }
    }
}

fn main() {
    yew::start_app::<Model>();
    // yew::start_app_with_props::<Model>(..).
}


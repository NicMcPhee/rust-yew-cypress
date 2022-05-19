use yew::prelude::*;

enum Msg {
    AddOne,
    AddTwo,
    AddThree,
    Double,
    Reset
}

struct CounterComponent {
    count: i64
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render the component
            }
            Msg::AddTwo => {
                self.count += 2;
                true
            }
            Msg::AddThree => {
                self.count += 3;
                true
            }
            Msg::Double => {
                self.count *= 2;
                true
            }
            Msg::Reset => {
                self.count = 0;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p data-cy="counter-display">{ self.count }</p>
                <button data-cy="add-one-button" onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button data-cy="add-two-button" onclick={link.callback(|_| Msg::AddTwo)}>{ "+2" }</button>
                <button data-cy="add-three-button" onclick={link.callback(|_| Msg::AddThree)}>{ "+3" }</button>
                <button data-cy="times-two-button" onclick={link.callback(|_| Msg::Double)}>{ "*2" }</button>
                <button data-cy="reset-button" onclick={link.callback(|_| Msg::Reset)}>{ "Reset" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}

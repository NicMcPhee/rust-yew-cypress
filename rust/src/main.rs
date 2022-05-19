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
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::AddTwo)}>{ "+2" }</button>
                // a button that adds 3 to the count
                <button onclick={link.callback(|_| Msg::AddThree)}>{ "+3" }</button>
                <button onclick={link.callback(|_| Msg::Double)}>{ "*2" }</button>
                <button onclick={link.callback(|_| Msg::Reset)}>{ "Reset" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}

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

#[cfg(test)]
mod tests {
    use mockall::*;
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn can_create_component() {
        let _component = CounterComponent {
            count: 0
        };
    }

    mock! {
        Context<T> {}     // Name of the mock struct, less the "Mock" prefix
        // impl Clone for MyStruct {   // specification of the trait to mock
        //     fn clone(&self) -> Self;
        // }
    }

    #[test]
    fn can_add_one() {
        let mut component = CounterComponent {
            count: 0
        };
        let mockContext = MockContext::new();
        let _ = component.update(&mockContext, Msg::AddOne);
        assert_eq!(component.count, 1);
    }
}
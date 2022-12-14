use body::Body;
use footer::Footer;
use header::Header;
use yew::prelude::*;

mod body;
mod footer;
mod header;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
              <Header />
              <Body />
              <Footer />
            </>
        }
    }
}

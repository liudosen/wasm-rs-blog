use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct Index {}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {};
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="uk-width-auto uk-height-viewport uk-overflow-auto">
                    <div class="uk-height-viewport uk-flex uk-flex-center uk-flex-middle uk-background-cover uk-light"
                         data-src="./static/pic/index_background.jpg" uk-img="">
                            <p>{"Welcome to 《Liudosen Blog》"}</p>
                    </div>
                </div>
            </>
        }
    }
}
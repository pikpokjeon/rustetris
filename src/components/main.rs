use crate::components::gamebox;

use yew::{function_component, html};

#[function_component(MainComponent)]
pub fn main_component() -> Html {
    html! {
        <main  class="flex justify-center ">
            <gamebox::GameBox/>
        </main>
    }
}

use sycamore::prelude::*;
// use log::debug;

mod kanban;
use kanban::board::Kanban as Kanban;

mod navbar;
use navbar::navbar::Navbar as Navbar;


#[component]
pub fn App<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        Navbar{}
        Kanban{}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    // let handle_button_click = |_| { debug!("hej")};
    
    sycamore::render(|cx| view! { cx,
        // button(on:click=handle_button_click){
        //     "Click me"
        // }

        div(class="app") {
            App{}
        }
    });
}


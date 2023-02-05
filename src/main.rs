use sycamore::prelude::*;
// use log::debug;

mod main_content;
use main_content::main_content::MainContent as MainContent;

mod navbar;
use navbar::navbar::Navbar as Navbar;

mod sidebar;
use sidebar::sidebar::Sidebar as Sidebar;


#[component]
pub fn App<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        Navbar{}
        Sidebar{}
        MainContent{}
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


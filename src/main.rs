use sycamore::prelude::*;
// use log::debug;

mod main_content;
use main_content::main_content::MainContent as MainContent;

mod navbar;
use navbar::navbar::Navbar as Navbar;

mod sidebar;
use sidebar::sidebar::Sidebar as Sidebar;

use sycamore_router::{ Route,Router, HistoryIntegration};

#[derive(PartialEq)]
#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/kanban")]
    Kanban,
    #[not_found]
    NotFound,
}

impl std::fmt::Display for AppRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            AppRoutes::Index => write!(f, "Index"),
            AppRoutes::Kanban => write!(f, "Kanban"),
            AppRoutes::NotFound => write!(f, "NotFound"),
        }
    }
}

#[component(inline_props)]
async fn Switch<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { cx,
        (match route.get().as_ref() {
            AppRoutes::Index => view! { cx, 
                Navbar(AppRoutes::Index)
                Sidebar{}
                MainContent(AppRoutes::Index)
                },
            AppRoutes::Kanban => view! { cx,  
                Navbar(AppRoutes::Kanban)
                Sidebar{}
                MainContent(AppRoutes::Kanban)
            },
            AppRoutes::NotFound => view! { cx,  
                Navbar(AppRoutes::NotFound)
                Sidebar{}
                MainContent(AppRoutes::NotFound)
            },
        })
    }
}


#[component]
pub fn App<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx: Scope, route: &ReadSignal<AppRoutes>| view! { cx,
                // div(class="app mb-2") {
                //     div(class="container mx-auto") {
                        Switch(route=route)
                //     }
                // }
            }
        )
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


use log::debug;
use sycamore::prelude::*;

use crate::AppRoutes;

#[component]
pub fn Navbar<T: Html>(cx: Scope, appRoutes: AppRoutes) -> View<T> {
    debug!("Navbar created");
    let state = create_signal(cx, appRoutes);

    view! { cx,
        div(class="navbar"){
            div(class="navbar__left"){
                div(class="navbar__item"){
                    (state.get())
                }
            }
        }
    }
}

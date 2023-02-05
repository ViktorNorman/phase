use sycamore::prelude::*;

#[component]
pub fn Navbar<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        div(class="navbar"){
            div(class="navbar__title"){
                "Kanban"
            }
        }
    }
}

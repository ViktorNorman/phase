use sycamore::prelude::*;

#[component]
pub fn Navbar<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        div(class="navbar"){
            div(class="navbar__left"){
                div(class="navbar__item"){
                    "Kanban board"
                }
                // div(class="navbar__item"){
                //     "Kanban"
                // }
                // div(class="navbar__item"){
                //     "Kanban"
                // }
            }
        }
    }
}

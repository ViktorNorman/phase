use sycamore::prelude::*;

#[component]
pub fn MainContent<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        div(class="main-content"){
            div(class="kanban__task"){

            }
        }
    }
}

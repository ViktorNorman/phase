use sycamore::prelude::*;

#[component]
pub fn Kanban<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        div(class="kanban__board"){
            div(class="kanban__task"){

            }
        }
    }
}

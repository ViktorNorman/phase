use sycamore::prelude::*;

#[component]
pub fn Sidebar<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        div(class="sidebar"){
            div(class="sidebar__top"){
                a(class="sidebar__header", href="/"){
                    "Phase"
                }
            }
            div(class="sidebar__bottom"){
                div(class="sidebar__views"){
                    "Views"
                    div(class="sidebar__items"){
                        a(class="sidebar__item", href="/kanban"){
                            "Kanban board"
                        }
                        a(class="sidebar__item", href="/planning"){
                            "Planning"
                        }
                        a(class="sidebar__item", href="/dependencies"){
                            "Dependencies"
                        }
                        a(class="sidebar__item", href="/change-log"){
                            "Change log"
                        }
                    }
                }
            div{
                "Settings"
            }
        }
    }
    }
}

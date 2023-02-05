use sycamore::prelude::*;

#[component]
pub fn Sidebar<T: Html>(cx: Scope) -> View<T> {
    view! { cx,
        div(class="sidebar"){
            div(class="sidebar__top"){
                div(class="sidebar__header"){
                    "Phase"
                }
            }
            div(class="sidebar__bottom"){
                div(class="sidebar__views"){
                    "Views"
                    div(class="sidebar__items"){
                        div(class="sidebar__item"){
                            "Kanban board"
                        }
                        div(class="sidebar__item"){
                            "Planning"
                        }
                        div(class="sidebar__item"){
                            "Dependencies"
                        }
                        div(class="sidebar__item"){
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

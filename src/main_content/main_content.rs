use sycamore::prelude::*;
use crate::AppRoutes;

#[component]
pub fn MainContent< T: Html>(cx: Scope, appRoutes: AppRoutes) -> View<T>{
    let state = create_signal(cx, appRoutes);

    view! { cx,
        div(class="main-content"){
            (if state.get() == AppRoutes::Index.into(){
                "INDEX!!!"
            } else{
                "NOT INDEX!"
            })
        }
    }
}

pub mod main_content;
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
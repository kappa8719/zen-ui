use sycamore_router::Route;

pub trait ShouldShowSidebar {
    fn should_show_sidebar(&self) -> bool {
        true
    }
}

#[derive(Route, Clone, Debug)]
pub enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/workspaces")]
    Workspaces,
    #[not_found]
    NotFound,
}

#[derive(Route, Clone, Debug)]
pub enum Routes {
    #[to("/auth")]
    Auth,
    #[to("/<_..>")]
    App(AppRoutes),
    #[not_found]
    NotFound,
}

impl ShouldShowSidebar for Routes {
    fn should_show_sidebar(&self) -> bool {
        match self {
            Routes::Auth => false,
            _ => true,
        }
    }
}

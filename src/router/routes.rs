use sycamore_router::Route;

#[derive(Route, Clone, Debug)]
pub enum Routes {
    #[to("/")]
    Index,
    #[to("/workspaces")]
    Workspaces,
    #[not_found]
    NotFound,
}
use crate::{app, router};
use sycamore::prelude::{ReadSignal, View};
use sycamore::{component, view};
use crate::router::Routes;

#[component(inline_props)]
pub fn Renderer(route: ReadSignal<Routes>) -> View {
    view! {
        (match route.get_clone() {
            Routes::Index => app::Page(),
            Routes::Workspaces => app::workspaces::Page(),
            Routes::NotFound => app::not_found::Page(),
        })
    }
}

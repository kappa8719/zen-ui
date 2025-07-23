use crate::router::{AppRoutes, Routes};
use crate::ui::components::sidebar::Sidebar;
use crate::web;
use std::mem::discriminant;
use sycamore::prelude::*;
use sycamore::prelude::{ReadSignal, View};
use sycamore::web::tags::*;
use sycamore::{component, view};

#[component(inline_props)]
fn RenderAppRoutes(route: ReadSignal<Routes>) -> View {
    let content = View::from_dynamic(move || {
        let Routes::App(app) = route.get_clone() else {
            return view!();
        };

        match app {
            AppRoutes::Home => web::app::Page(),
            AppRoutes::Workspaces => web::app::workspaces::Page(),
            AppRoutes::NotFound => web::not_found::Page(),
        }
    });

    web::app::Layout(Children::new(|| content))
}

#[component(inline_props)]
pub fn Renderer(route: ReadSignal<Routes>) -> View {
    let root = create_selector_with(
        move || route.get_clone(),
        |p, n| discriminant(p) == discriminant(n),
    );

    let view = View::from_dynamic(move || match root.get_clone() {
        Routes::Auth => web::auth::Page(),
        Routes::App(_) => view! { RenderAppRoutes(route = route) },
        Routes::NotFound => web::not_found::Page(),
    });

    view! {
        div {
            (view)
        }
    }
}

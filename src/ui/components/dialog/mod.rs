use sycamore::prelude::{Children, View};
use sycamore::{component, view};

#[component]
pub fn Dialog(children: Children) -> View {
    let children = children.call();
    view! {
        (children)
    }
}

#[component]
pub fn DialogContent(children: Children) -> View {
    view!()
}
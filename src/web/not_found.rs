use sycamore::prelude::*;
use sycamore::{component, view};

#[component]
pub fn Page() -> View {
    view! {
        div(class = "flex justify-center items-center size-full") {
            div {
                div(class = "text-3xl") {
                    "404 Not Found :("
                }
                p {
                    "The page was not found"
                }
            }
        }
    }
}

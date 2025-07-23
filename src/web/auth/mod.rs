use sycamore::prelude::*;
use sycamore::{component, view};

#[component]
pub fn Page() -> View {
    view! {
        div(class = "w-dvw h-dvh fixed flex") {
            div(class = "flex justify-center items-center size-full") {
                div(class = "card bg-base-100 w-96 shadow-sm") {
                    div(class = "card-body items-center") {
                        h2(class = "card-title") { "Sign In" }
                        p { "Identify" }
                        input(r#type = "text", class = "input")
                    }
                }
            }
        }
    }
}

use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("toggle.md"),
        |name| match name {
            "toggle_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <ToggleBasic /> }),
                source: extract_demo(include_str!("demos/toggle_basic.rs")),
            }),
            _ => None,
        },
    )
}

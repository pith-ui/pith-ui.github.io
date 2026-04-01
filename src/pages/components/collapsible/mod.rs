use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("collapsible.md"),
        |name| match name {
            "collapsible_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <CollapsibleBasic /> }),
                source: extract_demo(include_str!("demos/collapsible_basic.rs")),
            }),
            _ => None,
        },
    )
}

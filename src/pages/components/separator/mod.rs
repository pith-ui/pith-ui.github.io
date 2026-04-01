use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("separator.md"),
        |name| match name {
            "separator_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <SeparatorBasic /> }),
                source: extract_demo(include_str!("demos/separator_basic.rs")),
            }),
            _ => None,
        },
    )
}

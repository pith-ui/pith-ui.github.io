use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("tooltip.md"),
        |name| match name {
            "tooltip_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <TooltipBasic /> }),
                source: extract_demo(include_str!("demos/tooltip_basic.rs")),
            }),
            _ => None,
        },
    )
}

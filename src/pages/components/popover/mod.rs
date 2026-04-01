use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("popover.md"),
        |name| match name {
            "popover_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <PopoverBasic /> }),
                source: extract_demo(include_str!("demos/popover_basic.rs")),
            }),
            "popover_hover" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <PopoverHover /> }),
                source: extract_demo(include_str!("demos/popover_hover.rs")),
            }),
            _ => None,
        },
    )
}

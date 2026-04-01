use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("switch.md"),
        |name| match name {
            "switch_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <SwitchBasic /> }),
                source: extract_demo(include_str!("demos/switch_basic.rs")),
            }),
            _ => None,
        },
    )
}

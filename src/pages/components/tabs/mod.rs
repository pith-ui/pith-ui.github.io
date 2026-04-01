use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("tabs.md"),
        |name| match name {
            "tabs_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <TabsBasic /> }),
                source: extract_demo(include_str!("demos/tabs_basic.rs")),
            }),
            _ => None,
        },
    )
}

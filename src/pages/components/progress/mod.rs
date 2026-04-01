use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("progress.md"),
        |name| match name {
            "progress_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <ProgressBasic /> }),
                source: extract_demo(include_str!("demos/progress_basic.rs")),
            }),
            _ => None,
        },
    )
}

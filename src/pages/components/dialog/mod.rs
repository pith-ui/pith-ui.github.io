use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn DialogPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("dialog.md"),
        |name| match name {
            "dialog_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <DialogBasic /> }),
                source: extract_demo(include_str!("demos/dialog_basic.rs")),
            }),
            "dialog_nested" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <DialogNested /> }),
                source: extract_demo(include_str!("demos/dialog_nested.rs")),
            }),
            _ => None,
        },
    )
}

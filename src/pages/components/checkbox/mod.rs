use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("checkbox.md"),
        |name| match name {
            "checkbox_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <CheckboxBasic /> }),
                source: extract_demo(include_str!("demos/checkbox_basic.rs")),
            }),
            "checkbox_indeterminate" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <CheckboxIndeterminate /> }),
                source: extract_demo(include_str!("demos/checkbox_indeterminate.rs")),
            }),
            "checkbox_form" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <CheckboxForm /> }),
                source: extract_demo(include_str!("demos/checkbox_form.rs")),
            }),
            _ => None,
        },
    )
}

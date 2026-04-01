use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn AccordionPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("accordion.md"),
        |name| match name {
            "accordion_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <AccordionBasic /> }),
                source: extract_demo(include_str!("demos/accordion_basic.rs")),
            }),
            "accordion_multiple" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <AccordionMultipleDemo /> }),
                source: extract_demo(include_str!("demos/accordion_multiple.rs")),
            }),
            _ => None,
        },
    )
}

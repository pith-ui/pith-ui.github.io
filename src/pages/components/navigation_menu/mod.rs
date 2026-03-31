use leptos::prelude::*;

use crate::components::extract_demo;
use crate::components::md_page::{self, DemoEntry};

mod demos;
use demos::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    md_page::render_md_page(
        include_str!("navigation_menu.md"),
        |name| match name {
            "navigation_menu_basic" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <NavigationMenuBasic /> }),
                source: extract_demo(include_str!("demos/navigation_menu_basic.rs")),
            }),
            "navigation_menu_nested" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <NavigationMenuNested /> }),
                source: extract_demo(include_str!("demos/navigation_menu_nested.rs")),
            }),
            "navigation_menu_inline" => Some(DemoEntry {
                view: ViewFn::from(|| view! { <NavigationMenuInline /> }),
                source: extract_demo(include_str!("demos/navigation_menu_inline.rs")),
            }),
            _ => None,
        },
    )
}

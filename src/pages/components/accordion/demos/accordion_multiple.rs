// #region demo
use leptos::prelude::*;
use pith_ui::accordion::*;

const ROOT: &str = "w-full max-w-md divide-y divide-slate-200 rounded-lg border border-slate-200";
const TRIGGER: &str = "flex w-full items-center justify-between px-4 py-3 text-left text-sm \
    font-medium text-slate-900 hover:bg-slate-50 transition-colors \
    [&[data-state=open]>span]:rotate-180";
const CHEVRON: &str = "text-slate-500 transition-transform duration-200";
const CONTENT: &str = "accordion-content overflow-hidden";
const CONTENT_INNER: &str = "px-4 pb-3 text-sm text-slate-600";

fn chevron_down() -> impl IntoView {
    view! {
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m4 6 4 4 4-4" />
        </svg>
    }
}

#[component]
pub fn AccordionMultipleDemo() -> impl IntoView {
    view! {
        <AccordionMultiple
            default_values=vec!["item-1".to_string()]
            attr:class=ROOT
        >
            <AccordionItem value="item-1".to_string()>
                <AccordionHeader>
                    <AccordionTrigger attr:class=TRIGGER>
                        "What is Pith UI?"
                        <span class=CHEVRON>{chevron_down()}</span>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=CONTENT>
                    <div class=CONTENT_INNER>
                        "Pith UI is a library of high-quality unstyled Leptos components for design systems and web apps."
                    </div>
                </AccordionContent>
            </AccordionItem>

            <AccordionItem value="item-2".to_string()>
                <AccordionHeader>
                    <AccordionTrigger attr:class=TRIGGER>
                        "How do I get started?"
                        <span class=CHEVRON>{chevron_down()}</span>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=CONTENT>
                    <div class=CONTENT_INNER>
                        "Head to the \"Quick start\" guide in the docs. If you've used unstyled libraries before, you'll feel at home."
                    </div>
                </AccordionContent>
            </AccordionItem>

            <AccordionItem value="item-3".to_string()>
                <AccordionHeader>
                    <AccordionTrigger attr:class=TRIGGER>
                        "Can I use it for my project?"
                        <span class=CHEVRON>{chevron_down()}</span>
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class=CONTENT>
                    <div class=CONTENT_INNER>
                        "Of course! Pith UI is free and open source."
                    </div>
                </AccordionContent>
            </AccordionItem>
        </AccordionMultiple>
    }
}
// #endregion demo

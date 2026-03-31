use leptos::prelude::*;
use pith_ui::accordion::{AccordionContent, AccordionHeader, AccordionItem, AccordionMultiple, AccordionTrigger};
use pith_ui::scroll_area::{
    Orientation, ScrollArea, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaType,
    ScrollAreaViewport,
};
use pith_ui::separator::Separator;
use pith_ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

/// Extracts the content between `// #region demo` and `// #endregion demo` markers.
pub fn extract_demo(source: &str) -> String {
    let start_marker = "// #region demo";
    let end_marker = "// #endregion demo";

    let start = source
        .find(start_marker)
        .map(|i| i + start_marker.len())
        .unwrap_or(0);
    let end = source.find(end_marker).unwrap_or(source.len());

    source[start..end].trim().to_string()
}

/// Calls `Prism.highlightElement(el)` if Prism is loaded.
pub(crate) fn highlight_code(el: &web_sys::Element) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(prism) = js_sys::Reflect::get(&window, &"Prism".into()) else {
        return;
    };
    if prism.is_undefined() {
        return;
    }
    let Ok(func) = js_sys::Reflect::get(&prism, &"highlightElement".into()) else {
        return;
    };
    if let Some(func) = func.dyn_ref::<js_sys::Function>() {
        let _ = func.call1(&wasm_bindgen::JsValue::NULL, el);
    }
}

/// A button that copies text to the clipboard. Shows a check mark briefly after copying.
#[component]
pub(crate) fn CopyButton(#[prop(into)] text: String) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let text = StoredValue::new(text);

    let on_click = move |_: leptos::ev::MouseEvent| {
        if let Some(window) = web_sys::window() {
            if let Ok(nav) = js_sys::Reflect::get(&window, &JsValue::from_str("navigator")) {
                if let Ok(clip) = js_sys::Reflect::get(&nav, &JsValue::from_str("clipboard")) {
                    if let Ok(write_fn) =
                        js_sys::Reflect::get(&clip, &JsValue::from_str("writeText"))
                    {
                        if let Some(write_fn) = write_fn.dyn_ref::<js_sys::Function>() {
                            let _ = write_fn.call1(&clip, &JsValue::from_str(&text.get_value()));
                            set_copied.set(true);
                            set_timeout(move || set_copied.set(false), std::time::Duration::from_millis(1500));
                        }
                    }
                }
            }
        }
    };

    view! {
        <button
            on:click=on_click
            class="absolute top-2 right-2 z-10 flex items-center justify-center rounded-md p-1.5 \
                text-slate-400 hover:text-slate-200 hover:bg-white/10 transition-colors"
            title="Copy to clipboard"
        >
            {move || if copied.get() {
                view! {
                    <svg class="h-4 w-4 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                    </svg>
                }.into_any()
            } else {
                view! {
                    <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                    </svg>
                }.into_any()
            }}
        </button>
    }
}

const TAB_TRIGGER: &str = "px-3 py-1.5 text-sm font-medium rounded-md transition-colors \
    text-slate-500 hover:text-slate-700 \
    data-[state=active]:bg-white data-[state=active]:text-accent-700 data-[state=active]:shadow-sm";

/// A tabbed demo viewer: "Preview" renders the live component, "Code" shows the source.
#[component]
pub fn DemoTabs(#[prop(into)] source: String, children: ChildrenFn) -> impl IntoView {
    let code_ref = NodeRef::<leptos::html::Code>::new();
    let children = StoredValue::new(children);
    let source = StoredValue::new(source);

    Effect::new(move |_| {
        if let Some(el) = code_ref.get() {
            highlight_code(&el);
        }
    });

    view! {
        <Tabs default_value="preview".to_string() attr:class="rounded-lg border border-slate-200">
            <TabsList attr:class="flex gap-1 bg-slate-100 p-1 rounded-t-lg border-b border-slate-200">
                <TabsTrigger value="preview".to_string() attr:class=TAB_TRIGGER>
                    "Preview"
                </TabsTrigger>
                <TabsTrigger value="code".to_string() attr:class=TAB_TRIGGER>
                    "Code"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="preview".to_string() attr:class="p-6 bg-white">
                {children.get_value()()}
            </TabsContent>
            <TabsContent value="code".to_string() force_mount=true attr:class="data-[state=inactive]:hidden bg-code rounded-b-lg overflow-hidden">
                <div class="relative">
                    <CopyButton text=source.get_value() />
                    <ScrollArea r#type=ScrollAreaType::Auto attr:class="bg-code">
                        <ScrollAreaViewport attr:class="max-h-[680px]">
                            <pre class="m-0 bg-code p-4 pr-10 text-xs leading-relaxed">
                                <code node_ref=code_ref class="language-rust font-mono">
                                    {source.get_value()}
                                </code>
                            </pre>
                        </ScrollAreaViewport>
                    <ScrollAreaScrollbar
                        orientation=Orientation::Vertical
                        attr:class="flex w-2.5 touch-none select-none bg-transparent p-0.5 transition-colors"
                    >
                        <ScrollAreaThumb attr:class="relative flex-1 rounded-full bg-slate-500 hover:bg-slate-400 transition-colors">" "</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                    <ScrollAreaScrollbar
                        orientation=Orientation::Horizontal
                        attr:class="flex h-2.5 touch-none select-none bg-transparent p-0.5 transition-colors"
                    >
                        <ScrollAreaThumb attr:class="relative flex-1 rounded-full bg-slate-500 hover:bg-slate-400 transition-colors">" "</ScrollAreaThumb>
                    </ScrollAreaScrollbar>
                </ScrollArea>
                </div>
            </TabsContent>
        </Tabs>
    }
}

/// A code example block with copy button.
#[component]
pub fn CodeBlock(code: &'static str) -> impl IntoView {
    view! {
        <div class="relative">
            <CopyButton text=code />
            <pre class="overflow-x-auto rounded-lg bg-slate-900 p-4 pr-10 text-xs leading-relaxed">
                <code class="font-mono text-slate-100">{code}</code>
            </pre>
        </div>
    }
}

/// A badge showing a component feature.
#[component]
pub fn FeatureBadge(text: &'static str) -> impl IntoView {
    view! {
        <span class="inline-flex items-center rounded-full bg-accent-50 px-2.5 py-0.5 text-xs font-medium text-accent-700">
            {text}
        </span>
    }
}

/// Header for a component documentation page.
#[component]
pub fn ComponentPageHeader(
    title: &'static str,
    description: &'static str,
    #[prop(default = &[])] features: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div class="mb-10">
            <h1 class="text-3xl font-bold tracking-tight text-slate-900">{title}</h1>
            <p class="mt-2 text-lg text-slate-600">{description}</p>
            {(!features.is_empty()).then(|| view! {
                <div class="mt-4 flex flex-wrap gap-2">
                    {features.iter().map(|f| view! { <FeatureBadge text=f /> }).collect_view()}
                </div>
            })}
            <Separator
                attr:class="mt-6 h-px bg-slate-200"
                decorative=true
            />
        </div>
    }
}

/// A section within a component documentation page.
#[component]
pub fn DocSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="mb-12">
            <h2 class="mb-4 text-xl font-semibold text-slate-900">{title}</h2>
            {children()}
        </section>
    }
}

/// A part/sub-component description in the API reference.
#[component]
pub fn PartItem(name: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-baseline gap-3 py-2">
            <code class="shrink-0 rounded bg-slate-100 px-1.5 py-0.5 text-sm font-medium text-accent-700">{name}</code>
            <span class="text-sm text-slate-600">{description}</span>
        </div>
    }
}

/// API reference block for a single component part.
///
/// Use `<ApiPartProps>` and `<ApiPartDataAttrs>` as children to provide props
/// and data attribute tables:
///
/// ```rust,ignore
/// <ApiPart name="Root" description="..." renders="<nav>">
///     <ApiPartProps>
///         <Prop name="value" r#type="MaybeProp<String>" description="..." />
///     </ApiPartProps>
///     <ApiPartDataAttrs>
///         <DataAttr name="data-state" description="..." />
///     </ApiPartDataAttrs>
/// </ApiPart>
/// ```
#[component]
pub fn ApiPart(
    name: &'static str,
    description: &'static str,
    #[prop(default = "")] renders: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="border-t border-slate-200 pt-6 pb-2">
            <h3 class="text-lg font-semibold text-slate-900">{name}</h3>
            <p class="mt-1 text-sm text-slate-600">{description}</p>
            {(!renders.is_empty()).then(|| view! {
                <p class="mt-0.5 text-sm text-slate-500">
                    "Renders a "<code class="rounded bg-slate-100 px-1 py-0.5 text-xs font-medium text-slate-700">{renders}</code>" element."
                </p>
            })}
            {children.map(|c| c())}
        </div>
    }
}

/// Accordion-based props table. Use `<Prop />` components as children.
#[component]
pub fn ApiPartProps(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <div class="mt-4 overflow-hidden rounded-lg border border-slate-200">
            <div class=format!("{API_GRID_3} px-3 py-2 bg-slate-50 border-b border-slate-200")>
                <div class=API_TH>"Prop"</div>
                <div class=API_TH>"Type"</div>
                <div class=API_TH>"Default"</div>
            </div>
            <AccordionMultiple attr:class="divide-y divide-slate-100">
                {move || children.with_value(|c| c())}
            </AccordionMultiple>
        </div>
    }
}

/// Data attributes table. Use `<DataAttr />` components as children.
#[component]
pub fn ApiPartDataAttrs(children: Children) -> impl IntoView {
    view! {
        <div class="mt-4 overflow-hidden rounded-lg border border-slate-200">
            <div class=format!("{API_GRID_ATTRS} px-3 py-2 bg-slate-50 border-b border-slate-200")>
                <div class=API_TH>"Attribute"</div>
                <div class=API_TH>"Description"</div>
            </div>
            <div class="divide-y divide-slate-100">
                {children()}
            </div>
        </div>
    }
}

const API_GRID_3: &str = "grid grid-cols-[35%_45%_20%]";
const API_GRID_ATTRS: &str = "grid grid-cols-[35%_1fr]";
const API_GRID_2: &str = "grid grid-cols-[35%_1fr] gap-y-2";
const API_TH: &str = "text-xs font-semibold text-slate-700";
const API_TRIGGER: &str = "w-full px-3 py-2.5 text-left text-sm hover:bg-slate-50 \
    transition-colors cursor-pointer data-[state=open]:bg-slate-50/50";
const API_DETAIL: &str = "bg-slate-50/30 px-3 py-3 border-t border-slate-100";
const API_DT: &str = "text-xs font-medium text-slate-500 py-0.5";
const API_DD: &str = "text-sm text-slate-600 py-0.5";

/// A single prop definition. Renders as an accordion item inside `ApiPart`'s props table.
#[component]
pub fn Prop(
    name: &'static str,
    r#type: &'static str,
    #[prop(default = "\u{2014}")] default: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <AccordionItem value=name.to_string()>
            <AccordionHeader>
                <AccordionTrigger attr:class=format!("{API_GRID_3} {API_TRIGGER}")>
                    <div class="min-w-0">
                        <code class="text-xs font-medium text-accent-700">{name}</code>
                    </div>
                    <div class="min-w-0">
                        <code class="text-xs text-slate-500 break-all">{r#type}</code>
                    </div>
                    <div class="min-w-0 flex items-center justify-between gap-1">
                        <code class="text-xs text-slate-500">{default}</code>
                        <svg
                            class="h-3 w-3 shrink-0 text-slate-400 transition-transform duration-200 \
                                [[data-state=open]_&]:rotate-180"
                            viewBox="0 0 10 10"
                            fill="none"
                        >
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </div>
                </AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class="accordion-content overflow-hidden">
                <div class=API_DETAIL>
                    <dl class=API_GRID_2>
                        <dt class=API_DT>"Name"</dt>
                        <dd class=API_DD>
                            <code class="text-xs font-medium text-accent-700">{name}</code>
                        </dd>
                        <dt class=API_DT>"Description"</dt>
                        <dd class=API_DD>{description}</dd>
                        <dt class=API_DT>"Type"</dt>
                        <dd class=API_DD>
                            <code class="text-xs text-slate-500">{r#type}</code>
                        </dd>
                        <dt class=API_DT>"Default"</dt>
                        <dd class=API_DD>
                            <code class="text-xs text-slate-500">{default}</code>
                        </dd>
                    </dl>
                </div>
            </AccordionContent>
        </AccordionItem>
    }
}

/// A single data attribute row for `ApiPart`'s data_attrs slot.
#[component]
pub fn DataAttr(
    name: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("{API_GRID_ATTRS} px-3 py-2 text-sm")>
            <div><code class="text-xs font-medium text-slate-700">{name}</code></div>
            <div class="text-slate-600">{description}</div>
        </div>
    }
}

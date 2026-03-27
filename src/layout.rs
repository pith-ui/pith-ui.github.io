use leptos::prelude::*;
use leptos_router::{
    components::{A, Outlet},
    hooks::use_location,
};

/// Skip-to-content link — first focusable element on the page.
#[component]
pub fn SkipLink() -> impl IntoView {
    view! {
        <a
            href="#main-content"
            class="sr-only focus:not-sr-only focus:fixed focus:left-4 focus:top-4 focus:z-[100] focus:rounded-lg focus:bg-accent-600 focus:px-4 focus:py-2 focus:text-sm focus:font-semibold focus:text-white focus:shadow-lg"
        >
            "Skip to main content"
        </a>
    }
}

#[component]
pub fn SiteHeader() -> impl IntoView {
    view! {
        <SkipLink />
        <header class="sticky top-0 z-50 border-b border-slate-200 bg-white/80 backdrop-blur-sm">
            <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-6">
                <A href="/" attr:class="flex items-center gap-2 text-xl font-bold text-slate-900 no-underline hover:text-accent-600 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 rounded-sm transition-colors">
                    "Pith UI"
                </A>
                <nav aria-label="Main" class="flex items-center gap-6">
                    <A href="/docs/getting-started" attr:class="text-sm font-medium text-slate-600 no-underline hover:text-slate-900 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 rounded-sm transition-colors">
                        "Docs"
                    </A>
                    <A href="/docs/components" attr:class="text-sm font-medium text-slate-600 no-underline hover:text-slate-900 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 rounded-sm transition-colors">
                        "Components"
                    </A>
                    <a
                        href="https://github.com/pith-ui/pith-ui"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm font-medium text-slate-600 no-underline hover:text-slate-900 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 rounded-sm transition-colors"
                    >
                        "GitHub"
                    </a>
                </nav>
            </div>
        </header>
    }
}

struct NavItem {
    label: &'static str,
    href: &'static str,
}

struct NavGroup {
    title: &'static str,
    items: &'static [NavItem],
}

const NAV_GROUPS: &[NavGroup] = &[
    NavGroup {
        title: "Getting Started",
        items: &[
            NavItem { label: "Introduction", href: "/docs/getting-started" },
        ],
    },
    NavGroup {
        title: "Layout",
        items: &[
            NavItem { label: "Aspect Ratio", href: "/docs/components/aspect-ratio" },
            NavItem { label: "Scroll Area", href: "/docs/components/scroll-area" },
            NavItem { label: "Separator", href: "/docs/components/separator" },
        ],
    },
    NavGroup {
        title: "Inputs",
        items: &[
            NavItem { label: "Calendar", href: "/docs/components/calendar" },
            NavItem { label: "Checkbox", href: "/docs/components/checkbox" },
            NavItem { label: "Combobox", href: "/docs/components/combobox" },
            NavItem { label: "Form", href: "/docs/components/form" },
            NavItem { label: "Label", href: "/docs/components/label" },
            NavItem { label: "OTP Field", href: "/docs/components/otp-field" },
            NavItem { label: "Password Toggle Field", href: "/docs/components/password-toggle-field" },
            NavItem { label: "Radio Group", href: "/docs/components/radio-group" },
            NavItem { label: "Select", href: "/docs/components/select" },
            NavItem { label: "Slider", href: "/docs/components/slider" },
            NavItem { label: "Switch", href: "/docs/components/switch" },
            NavItem { label: "Time Field", href: "/docs/components/time-field" },
            NavItem { label: "Toggle", href: "/docs/components/toggle" },
            NavItem { label: "Toggle Group", href: "/docs/components/toggle-group" },
        ],
    },
    NavGroup {
        title: "Overlays",
        items: &[
            NavItem { label: "Alert Dialog", href: "/docs/components/alert-dialog" },
            NavItem { label: "Context Menu", href: "/docs/components/context-menu" },
            NavItem { label: "Dialog", href: "/docs/components/dialog" },
            NavItem { label: "Dropdown Menu", href: "/docs/components/dropdown-menu" },
            NavItem { label: "Hover Card", href: "/docs/components/hover-card" },
            NavItem { label: "Popover", href: "/docs/components/popover" },
            NavItem { label: "Toast", href: "/docs/components/toast" },
            NavItem { label: "Tooltip", href: "/docs/components/tooltip" },
        ],
    },
    NavGroup {
        title: "Navigation",
        items: &[
            NavItem { label: "Menubar", href: "/docs/components/menubar" },
            NavItem { label: "Navigation Menu", href: "/docs/components/navigation-menu" },
            NavItem { label: "Toolbar", href: "/docs/components/toolbar" },
        ],
    },
    NavGroup {
        title: "Disclosure",
        items: &[
            NavItem { label: "Accordion", href: "/docs/components/accordion" },
            NavItem { label: "Collapsible", href: "/docs/components/collapsible" },
            NavItem { label: "Tabs", href: "/docs/components/tabs" },
        ],
    },
    NavGroup {
        title: "Data Display",
        items: &[
            NavItem { label: "Avatar", href: "/docs/components/avatar" },
            NavItem { label: "Progress", href: "/docs/components/progress" },
        ],
    },
    NavGroup {
        title: "Utilities",
        items: &[
            NavItem { label: "Accessible Icon", href: "/docs/components/accessible-icon" },
            NavItem { label: "Menu", href: "/docs/components/menu" },
        ],
    },
];

#[component]
pub fn DocsLayout() -> impl IntoView {
    let location = use_location();

    view! {
        <div class="mx-auto flex max-w-7xl">
            <aside class="sticky top-16 hidden h-[calc(100vh-4rem)] w-64 shrink-0 overflow-y-auto border-r border-slate-200 px-4 py-8 md:block">
                <nav aria-label="Docs sidebar" class="space-y-6">
                    {NAV_GROUPS.iter().map(|group| {
                        view! {
                            <div role="group" aria-labelledby=format!("nav-group-{}", group.title.to_lowercase().replace(' ', "-"))>
                                <p
                                    id=format!("nav-group-{}", group.title.to_lowercase().replace(' ', "-"))
                                    class="mb-2 text-xs font-semibold uppercase tracking-wider text-slate-500"
                                >
                                    {group.title}
                                </p>
                                <ul class="space-y-1" role="list">
                                    {group.items.iter().map(|item| {
                                        let href = item.href;
                                        let label = item.label;
                                        let is_active = move || location.pathname.get() == href;
                                        view! {
                                            <li>
                                                <A
                                                    href=href
                                                    attr:aria-current=move || if is_active() { Some("page") } else { None }
                                                    attr:class=move || {
                                                        if is_active() {
                                                            "block rounded-md bg-accent-50 px-3 py-1.5 text-sm font-medium text-accent-700 no-underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2"
                                                        } else {
                                                            "block rounded-md px-3 py-1.5 text-sm text-slate-700 no-underline hover:bg-slate-50 hover:text-slate-900 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 transition-colors"
                                                        }
                                                    }
                                                >
                                                    {label}
                                                </A>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }
                    }).collect_view()}
                </nav>
            </aside>
            <main id="main-content" class="min-w-0 flex-1 px-8 py-8 lg:px-12">
                <Outlet />
            </main>
        </div>
    }
}

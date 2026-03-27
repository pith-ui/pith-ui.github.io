use leptos::prelude::*;
use leptos_router::components::A;
use pith_ui::separator::Separator;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main id="main-content">
            // Hero
            <section class="relative overflow-hidden border-b border-slate-200 bg-gradient-to-b from-accent-50/50 to-white">
                <div class="mx-auto max-w-7xl px-6 py-24 lg:py-32">
                    <div class="mx-auto max-w-2xl text-center">
                        <h1 class="text-5xl font-bold tracking-tight text-slate-900 lg:text-6xl">
                            "Accessible UI primitives for "
                            <span class="text-accent-600">"Leptos"</span>
                        </h1>
                        <p class="mt-6 text-lg leading-8 text-slate-600">
                            "An open-source library of unstyled, accessible, and composable UI components. Build high-quality web applications with full control over styling."
                        </p>
                        <div class="mt-10 flex items-center justify-center gap-4">
                            <A
                                href="/docs/getting-started"
                                attr:class="rounded-lg bg-accent-600 px-5 py-2.5 text-sm font-semibold text-white no-underline shadow-sm hover:bg-accent-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 transition-colors"
                            >
                                "Get Started"
                            </A>
                            <A
                                href="/docs/components"
                                attr:class="rounded-lg border border-slate-300 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 no-underline shadow-sm hover:bg-slate-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 transition-colors"
                            >
                                "View Components"
                            </A>
                        </div>
                    </div>
                </div>
            </section>

            // Features
            <section class="py-20">
                <div class="mx-auto max-w-7xl px-6">
                    <div class="mx-auto max-w-2xl text-center">
                        <h2 class="text-3xl font-bold tracking-tight text-slate-900">"Built for the modern web"</h2>
                        <p class="mt-4 text-lg text-slate-600">"Everything you need to build accessible, production-quality interfaces."</p>
                    </div>
                    <div class="mx-auto mt-16 grid max-w-5xl gap-8 sm:grid-cols-2 lg:grid-cols-4">
                        <FeatureCard
                            title="Accessible"
                            description="WAI-ARIA compliant. Keyboard navigation, focus management, and screen reader support built in."
                        />
                        <FeatureCard
                            title="Composable"
                            description="Compound component pattern. Mix and match parts for complete control over structure and behavior."
                        />
                        <FeatureCard
                            title="Unstyled"
                            description="Zero styling opinions. Use Tailwind, CSS modules, or any approach you prefer."
                        />
                        <FeatureCard
                            title="Idiomatic"
                            description="Native Leptos APIs with signals, context, and reactivity. Not a React wrapper."
                        />
                    </div>
                </div>
            </section>

            <Separator attr:class="mx-auto h-px max-w-5xl bg-slate-200" decorative=true />

            // Component Preview
            <section class="py-20">
                <div class="mx-auto max-w-7xl px-6">
                    <div class="mx-auto max-w-2xl text-center">
                        <h2 class="text-3xl font-bold tracking-tight text-slate-900">"35+ components"</h2>
                        <p class="mt-4 text-lg text-slate-600">"From simple toggles to complex comboboxes, every component you need."</p>
                    </div>
                    <div class="mx-auto mt-12 grid max-w-5xl gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
                        <ComponentCard name="Accessible Icon" href="/docs/components/accessible-icon" />
                        <ComponentCard name="Accordion" href="/docs/components/accordion" />
                        <ComponentCard name="Alert Dialog" href="/docs/components/alert-dialog" />
                        <ComponentCard name="Aspect Ratio" href="/docs/components/aspect-ratio" />
                        <ComponentCard name="Avatar" href="/docs/components/avatar" />
                        <ComponentCard name="Calendar" href="/docs/components/calendar" />
                        <ComponentCard name="Checkbox" href="/docs/components/checkbox" />
                        <ComponentCard name="Collapsible" href="/docs/components/collapsible" />
                        <ComponentCard name="Combobox" href="/docs/components/combobox" />
                        <ComponentCard name="Context Menu" href="/docs/components/context-menu" />
                        <ComponentCard name="Dialog" href="/docs/components/dialog" />
                        <ComponentCard name="Dropdown Menu" href="/docs/components/dropdown-menu" />
                        <ComponentCard name="Form" href="/docs/components/form" />
                        <ComponentCard name="Hover Card" href="/docs/components/hover-card" />
                        <ComponentCard name="Label" href="/docs/components/label" />
                        <ComponentCard name="Menu" href="/docs/components/menu" />
                        <ComponentCard name="Menubar" href="/docs/components/menubar" />
                        <ComponentCard name="Navigation Menu" href="/docs/components/navigation-menu" />
                        <ComponentCard name="OTP Field" href="/docs/components/otp-field" />
                        <ComponentCard name="Password Toggle" href="/docs/components/password-toggle-field" />
                        <ComponentCard name="Popover" href="/docs/components/popover" />
                        <ComponentCard name="Progress" href="/docs/components/progress" />
                        <ComponentCard name="Radio Group" href="/docs/components/radio-group" />
                        <ComponentCard name="Scroll Area" href="/docs/components/scroll-area" />
                        <ComponentCard name="Select" href="/docs/components/select" />
                        <ComponentCard name="Separator" href="/docs/components/separator" />
                        <ComponentCard name="Slider" href="/docs/components/slider" />
                        <ComponentCard name="Switch" href="/docs/components/switch" />
                        <ComponentCard name="Tabs" href="/docs/components/tabs" />
                        <ComponentCard name="Time Field" href="/docs/components/time-field" />
                        <ComponentCard name="Toast" href="/docs/components/toast" />
                        <ComponentCard name="Toggle" href="/docs/components/toggle" />
                        <ComponentCard name="Toggle Group" href="/docs/components/toggle-group" />
                        <ComponentCard name="Toolbar" href="/docs/components/toolbar" />
                        <ComponentCard name="Tooltip" href="/docs/components/tooltip" />
                    </div>
                </div>
            </section>

            // Footer
            <footer class="border-t border-slate-200 py-12">
                <div class="mx-auto max-w-7xl px-6 text-center">
                    <p class="text-sm text-slate-600">"MIT License. Built with Leptos."</p>
                </div>
            </footer>
        </main>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="rounded-xl border border-slate-200 bg-white p-6">
            <h3 class="text-base font-semibold text-slate-900">{title}</h3>
            <p class="mt-2 text-sm leading-relaxed text-slate-600">{description}</p>
        </div>
    }
}

#[component]
fn ComponentCard(name: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="group rounded-lg border border-slate-200 bg-white px-4 py-3 text-sm font-medium text-slate-700 no-underline hover:border-accent-300 hover:bg-accent-50 hover:text-accent-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500 focus-visible:ring-offset-2 transition-all"
        >
            {name}
        </A>
    }
}

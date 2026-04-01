// #region demo
use leptos::prelude::*;
use pith_ui::tabs::*;

const TAB_LIST: &str = "relative z-0 flex gap-1 px-1 shadow-[inset_0_-1px] shadow-slate-200";
const TAB_TRIGGER: &str = "flex h-8 items-center justify-center border-0 px-2 text-sm font-medium \
    break-keep whitespace-nowrap text-slate-600 outline-hidden select-none \
    hover:text-slate-900 data-[state=active]:text-slate-900";
const TAB_PANEL: &str = "relative flex h-32 items-center justify-center \
    -outline-offset-1 outline-blue-800 focus-visible:rounded-md focus-visible:outline-2";

fn overview_icon() -> impl IntoView {
    view! {
        <svg class="size-10 text-slate-300" width="40" height="40" viewBox="0 0 30 30" fill="currentcolor">
            <path d="M 6 4 C 4.895 4 4 4.895 4 6 L 4 12 C 4 13.105 4.895 14 6 14 L 12 14 C 13.105 14 14 13.105 14 12 L 14 6 C 14 4.895 13.105 4 12 4 L 6 4 z M 18 4 C 16.895 4 16 4.895 16 6 L 16 12 C 16 13.105 16.895 14 18 14 L 24 14 C 25.105 14 26 13.105 26 12 L 26 6 C 26 4.895 25.105 4 24 4 L 18 4 z M 9 6 C 10.657 6 12 7.343 12 9 C 12 10.657 10.657 12 9 12 C 7.343 12 6 10.657 6 9 C 6 7.343 7.343 6 9 6 z M 18 6 L 24 6 L 24 12 L 18 12 L 18 6 z M 6 16 C 4.895 16 4 16.895 4 18 L 4 24 C 4 25.105 4.895 26 6 26 L 12 26 C 13.105 26 14 25.105 14 24 L 14 18 C 14 16.895 13.105 16 12 16 L 6 16 z M 18 16 C 16.895 16 16 16.895 16 18 L 16 24 C 16 25.105 16.895 26 18 26 L 24 26 C 25.105 26 26 25.105 26 24 L 26 18 C 26 16.895 25.105 16 24 16 L 18 16 z M 21 17.5 L 24.5 21 L 21 24.5 L 17.5 21 L 21 17.5 z M 9 18 L 11.886719 23 L 6.1132812 23 L 9 18 z" />
        </svg>
    }
}

fn lock_icon() -> impl IntoView {
    view! {
        <svg class="size-10 text-slate-300" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
            <path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
    }
}

fn bell_icon() -> impl IntoView {
    view! {
        <svg class="size-10 text-slate-300" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
            <path d="M13.73 21a2 2 0 0 1-3.46 0" />
        </svg>
    }
}

#[component]
pub fn TabsBasic() -> impl IntoView {
    view! {
        <Tabs
            default_value="account".to_string()
            attr:class="w-full max-w-md rounded-md border border-slate-200"
        >
            <TabsList attr:class=TAB_LIST>
                <TabsTrigger value="account".to_string() attr:class=TAB_TRIGGER>
                    "Account"
                </TabsTrigger>
                <TabsTrigger value="password".to_string() attr:class=TAB_TRIGGER>
                    "Password"
                </TabsTrigger>
                <TabsTrigger value="notifications".to_string() attr:class=TAB_TRIGGER>
                    "Notifications"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="account".to_string() attr:class=TAB_PANEL>
                {overview_icon()}
            </TabsContent>
            <TabsContent value="password".to_string() attr:class=TAB_PANEL>
                {lock_icon()}
            </TabsContent>
            <TabsContent value="notifications".to_string() attr:class=TAB_PANEL>
                {bell_icon()}
            </TabsContent>
        </Tabs>
    }
}
// #endregion demo

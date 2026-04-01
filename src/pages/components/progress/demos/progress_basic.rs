// #region demo
use leptos::prelude::*;
use pith_ui::progress::*;

const ROOT: &str = "grid w-48 grid-cols-2 gap-y-2";
const LABEL: &str = "text-sm font-medium text-slate-900";
const VALUE: &str = "col-start-2 text-right text-sm text-slate-900";
const TRACK: &str = "col-span-full h-1 overflow-hidden rounded-sm bg-slate-200 \
    shadow-[inset_0_0_0_1px] shadow-slate-200";
const INDICATOR: &str = "block h-full bg-slate-500 transition-all duration-500";

#[component]
pub fn ProgressBasic() -> impl IntoView {
    let (value, set_value) = signal(20u32);

    // Simulate progress
    set_interval(
        move || {
            set_value.update(|v| {
                *v = (*v + (js_sys::Math::random() * 25.0) as u32).min(100);
            });
        },
        std::time::Duration::from_millis(1000),
    );

    view! {
        <Progress value=Signal::derive(move || value.get() as f64) attr:class=ROOT>
            <span class=LABEL>"Export data"</span>
            <span class=VALUE>{move || format!("{}%", value.get())}</span>
            <div class=TRACK>
                <ProgressIndicator
                    attr:class=INDICATOR
                    attr:style=move || format!("width: {}%", value.get())
                />
            </div>
        </Progress>
    }
}
// #endregion demo

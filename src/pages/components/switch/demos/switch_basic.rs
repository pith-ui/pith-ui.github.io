// #region demo
use leptos::prelude::*;
use pith_ui::label::Label;
use pith_ui::switch::*;

const SWITCH: &str = "relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full \
    border-2 border-transparent bg-slate-200 transition-colors \
    data-[state=checked]:bg-accent-600";
const SWITCH_DISABLED: &str = "relative inline-flex h-6 w-11 shrink-0 cursor-not-allowed \
    rounded-full border-2 border-transparent bg-slate-100 opacity-50";
const THUMB: &str = "pointer-events-none block h-5 w-5 translate-x-0 rounded-full bg-white \
    shadow-sm ring-0 transition-transform data-[state=checked]:translate-x-5";
const THUMB_DISABLED: &str = "pointer-events-none block h-5 w-5 translate-x-0 rounded-full \
    bg-white shadow-sm ring-0";
const LABEL: &str = "text-sm font-medium text-slate-700 select-none";
const LABEL_DISABLED: &str = "text-sm text-slate-400 select-none";

#[component]
pub fn SwitchBasic() -> impl IntoView {
    let (airplane, set_airplane) = signal(false);
    let (wifi, set_wifi) = signal(true);

    view! {
        <div class="space-y-6 max-w-sm">
            <div class="flex items-center justify-between">
                <Label attr:r#for="airplane" attr:class=LABEL>
                    "Airplane Mode"
                </Label>
                <Switch
                    attr:id="airplane"
                    checked=airplane
                    on_checked_change=move |v| set_airplane.set(v)
                    attr:class=SWITCH
                >
                    <SwitchThumb attr:class=THUMB />
                </Switch>
            </div>

            <div class="flex items-center justify-between">
                <Label attr:r#for="wifi" attr:class=LABEL>
                    "Wi-Fi"
                </Label>
                <Switch
                    attr:id="wifi"
                    checked=wifi
                    on_checked_change=move |v| set_wifi.set(v)
                    attr:class=SWITCH
                >
                    <SwitchThumb attr:class=THUMB />
                </Switch>
            </div>

            <div class="flex items-center justify-between">
                <Label attr:r#for="disabled-switch" attr:class=LABEL_DISABLED>
                    "Disabled"
                </Label>
                <Switch
                    attr:id="disabled-switch"
                    disabled=true
                    attr:class=SWITCH_DISABLED
                >
                    <SwitchThumb attr:class=THUMB_DISABLED />
                </Switch>
            </div>
        </div>
    }
}
// #endregion demo

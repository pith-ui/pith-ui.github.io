// #region demo
use leptos::prelude::*;
use pith_ui::checkbox::*;
use pith_ui::label::Label;

const CHECKBOX: &str = "flex h-5 w-5 items-center justify-center rounded border \
    border-slate-300 bg-white transition-colors \
    data-[state=checked]:border-accent-600 data-[state=checked]:bg-accent-600";
const CHECKBOX_DISABLED: &str = "flex h-5 w-5 items-center justify-center rounded border \
    border-slate-200 bg-slate-100 opacity-50 cursor-not-allowed";
const INDICATOR: &str = "text-white";
const LABEL: &str = "text-sm font-medium text-slate-700 select-none cursor-pointer";
const LABEL_DISABLED: &str = "text-sm text-slate-400 select-none cursor-not-allowed";

fn check_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
            stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <path d="m3 7 3 3 5-5" />
        </svg>
    }
}

#[component]
pub fn CheckboxBasic() -> impl IntoView {
    let (checked, set_checked) = signal(CheckedState::True);

    view! {
        <div class="space-y-4">
            <div class="flex items-center gap-3">
                <Checkbox
                    attr:id="terms"
                    attr:class=CHECKBOX
                    checked=checked
                    on_checked_change=move |c| set_checked.set(c)
                >
                    <CheckboxIndicator attr:class=INDICATOR>
                        {check_icon()}
                    </CheckboxIndicator>
                </Checkbox>
                <Label attr:r#for="terms" attr:class=LABEL>
                    "Accept terms and conditions"
                </Label>
            </div>

            <div class="flex items-center gap-3">
                <Checkbox
                    attr:id="marketing"
                    default_checked=CheckedState::False
                    attr:class=CHECKBOX
                >
                    <CheckboxIndicator attr:class=INDICATOR>
                        {check_icon()}
                    </CheckboxIndicator>
                </Checkbox>
                <Label attr:r#for="marketing" attr:class=LABEL>
                    "Send me marketing emails"
                </Label>
            </div>

            <div class="flex items-center gap-3">
                <Checkbox
                    attr:id="disabled"
                    disabled=true
                    attr:class=CHECKBOX_DISABLED
                >
                    <CheckboxIndicator attr:class=INDICATOR>
                        {check_icon()}
                    </CheckboxIndicator>
                </Checkbox>
                <Label attr:r#for="disabled" attr:class=LABEL_DISABLED>
                    "Disabled option"
                </Label>
            </div>
        </div>
    }
}
// #endregion demo

// #region demo
use leptos::ev;
use leptos::prelude::*;
use pith_ui::checkbox::*;
use pith_ui::label::Label;
use wasm_bindgen::JsCast;

const CHECKBOX: &str = "flex h-5 w-5 items-center justify-center rounded border \
    border-slate-300 bg-white transition-colors \
    data-[state=checked]:border-accent-600 data-[state=checked]:bg-accent-600";
const INDICATOR: &str = "text-white";
const LABEL: &str = "text-sm font-medium text-slate-700 select-none cursor-pointer";
const BUTTON: &str = "rounded-md bg-accent-600 px-4 py-2 text-sm font-medium text-white \
    hover:bg-accent-700 transition-colors";

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
pub fn CheckboxForm() -> impl IntoView {
    let (submitted, set_submitted) = signal(String::new());

    view! {
        <form
            class="space-y-4 max-w-sm"
            on:submit=move |e: ev::SubmitEvent| {
                e.prevent_default();
                let form = e.target().unwrap().unchecked_into::<web_sys::HtmlFormElement>();
                let data = web_sys::FormData::new_with_form(&form).unwrap();
                let stay = data.get("stay-logged-in").as_string().unwrap_or_default();
                let news = data.get("newsletter").as_string().unwrap_or_default();
                set_submitted.set(format!("stay-logged-in={stay}, newsletter={news}"));
            }
        >
            <div class="flex items-center gap-3">
                <Checkbox
                    attr:id="stay"
                    name="stay-logged-in"
                    value="yes"
                    default_checked=CheckedState::True
                    attr:class=CHECKBOX
                >
                    <CheckboxIndicator attr:class=INDICATOR>
                        {check_icon()}
                    </CheckboxIndicator>
                </Checkbox>
                <Label attr:r#for="stay" attr:class=LABEL>
                    "Stay logged in for 7 days"
                </Label>
            </div>

            <div class="flex items-center gap-3">
                <Checkbox
                    attr:id="news"
                    name="newsletter"
                    value="yes"
                    attr:class=CHECKBOX
                >
                    <CheckboxIndicator attr:class=INDICATOR>
                        {check_icon()}
                    </CheckboxIndicator>
                </Checkbox>
                <Label attr:r#for="news" attr:class=LABEL>
                    "Subscribe to newsletter"
                </Label>
            </div>

            <button type="submit" class=BUTTON>"Submit"</button>

            <Show when=move || !submitted.get().is_empty()>
                <p class="text-sm text-slate-600">
                    "Submitted: " {move || submitted.get()}
                </p>
            </Show>
        </form>
    }
}
// #endregion demo

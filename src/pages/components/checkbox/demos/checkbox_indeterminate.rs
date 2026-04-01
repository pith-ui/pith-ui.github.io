// #region demo
use leptos::prelude::*;
use pith_ui::checkbox::*;
use pith_ui::label::Label;

const CHECKBOX: &str = "flex h-5 w-5 items-center justify-center rounded border \
    border-slate-300 bg-white transition-colors \
    data-[state=checked]:border-accent-600 data-[state=checked]:bg-accent-600 \
    data-[state=indeterminate]:border-accent-600 data-[state=indeterminate]:bg-accent-600";
const INDICATOR: &str = "text-white";
const LABEL: &str = "text-sm font-medium text-slate-700 select-none cursor-pointer";

fn check_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
            stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <path d="m3 7 3 3 5-5" />
        </svg>
    }
}

fn minus_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
            stroke="currentColor" stroke-width="2"
            stroke-linecap="round">
            <path d="M3 7h8" />
        </svg>
    }
}

#[component]
pub fn CheckboxIndeterminate() -> impl IntoView {
    let items = ["Lettuce", "Tomato", "Onion"];
    let (checked, set_checked) = signal([true, false, true]);

    let all_checked = Signal::derive(move || checked.get().iter().all(|&v| v));
    let some_checked = Signal::derive(move || checked.get().iter().any(|&v| v));

    let parent_state = Signal::derive(move || {
        if all_checked.get() {
            CheckedState::True
        } else if some_checked.get() {
            CheckedState::Indeterminate
        } else {
            CheckedState::False
        }
    });

    view! {
        <div class="space-y-3">
            <div class="flex items-center gap-3">
                <Checkbox
                    attr:id="select-all"
                    attr:class=CHECKBOX
                    checked=parent_state
                    on_checked_change=move |state| {
                        let val = state == CheckedState::True;
                        set_checked.set([val, val, val]);
                    }
                >
                    <CheckboxIndicator attr:class=INDICATOR>
                        {move || if all_checked.get() {
                            check_icon().into_any()
                        } else {
                            minus_icon().into_any()
                        }}
                    </CheckboxIndicator>
                </Checkbox>
                <Label attr:r#for="select-all" attr:class=LABEL>
                    "Select all toppings"
                </Label>
            </div>

            <div class="ml-6 space-y-2">
                {items.into_iter().enumerate().map(|(i, item)| {
                    let id = format!("topping-{i}");
                    let id_clone = id.clone();
                    view! {
                        <div class="flex items-center gap-3">
                            <Checkbox
                                attr:id=id.clone()
                                attr:class=CHECKBOX
                                checked=Signal::derive(move || {
                                    if checked.get()[i] { CheckedState::True } else { CheckedState::False }
                                })
                                on_checked_change=move |state| {
                                    let mut vals = checked.get();
                                    vals[i] = state == CheckedState::True;
                                    set_checked.set(vals);
                                }
                            >
                                <CheckboxIndicator attr:class=INDICATOR>
                                    {check_icon()}
                                </CheckboxIndicator>
                            </Checkbox>
                            <Label attr:r#for=id_clone attr:class=LABEL>
                                {item}
                            </Label>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
// #endregion demo

// #region demo
use leptos::prelude::*;
use pith_ui::popover::*;

const TRIGGER: &str = "flex size-10 items-center justify-center rounded-md border \
    border-slate-200 bg-slate-50 text-slate-900 select-none hover:bg-slate-100 \
    active:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors";
const CONTENT: &str = "popover-content rounded-lg bg-white px-6 py-4 text-slate-900 \
    shadow-lg shadow-slate-200 outline outline-1 outline-slate-200";
const ARROW: &str = "fill-white [filter:drop-shadow(0_1px_0_rgb(0_0_0/0.06))]";

fn bell_icon() -> impl IntoView {
    view! {
        <svg fill="currentcolor" width="20" height="20" viewBox="0 0 16 16" aria-label="Notifications">
            <path d="M 8 1 C 7.453125 1 7 1.453125 7 2 L 7 3.140625 C 5.28125 3.589844 4 5.144531 4 7 L 4 10.984375 C 4 10.984375 3.984375 11.261719 3.851563 11.519531 C 3.71875 11.78125 3.558594 12 3 12 L 3 13 L 13 13 L 13 12 C 12.40625 12 12.253906 11.78125 12.128906 11.53125 C 12.003906 11.277344 12 11.003906 12 11.003906 L 12 7 C 12 5.144531 10.71875 3.589844 9 3.140625 L 9 2 C 9 1.453125 8.546875 1 8 1 Z M 8 13 C 7.449219 13 7 13.449219 7 14 C 7 14.550781 7.449219 15 8 15 C 8.550781 15 9 14.550781 9 14 C 9 13.449219 8.550781 13 8 13 Z M 8 4 C 9.664063 4 11 5.335938 11 7 L 11 10.996094 C 11 10.996094 10.988281 11.472656 11.234375 11.96875 C 11.238281 11.980469 11.246094 11.988281 11.25 12 L 4.726563 12 C 4.730469 11.992188 4.738281 11.984375 4.742188 11.980469 C 4.992188 11.488281 5 11.015625 5 11.015625 L 5 7 C 5 5.335938 6.335938 4 8 4 Z" />
        </svg>
    }
}

#[component]
pub fn PopoverHover() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Popover open=open on_open_change=Callback::new(move |v| set_open.set(v))>
            <PopoverTrigger
                attr:class=TRIGGER
                on:pointerenter=move |_| set_open.set(true)
                on:pointerleave=move |_| set_open.set(false)
            >
                {bell_icon()}
            </PopoverTrigger>
            <PopoverPortal>
                <PopoverContent
                    attr:class=CONTENT
                    side_offset=8.0
                    on:pointerenter=move |_| set_open.set(true)
                    on:pointerleave=move |_| set_open.set(false)
                >
                    <PopoverArrow attr:class=ARROW />
                    <h3 class="text-base font-medium">"Notifications"</h3>
                    <p class="text-base text-slate-600">"You are all caught up. Good job!"</p>
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}
// #endregion demo

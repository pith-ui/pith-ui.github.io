// #region demo
use leptos::prelude::*;
use pith_ui::separator::*;

const HORIZONTAL: &str = "my-4 h-px bg-slate-200";
const VERTICAL: &str = "h-full w-px bg-slate-200";

#[component]
pub fn SeparatorBasic() -> impl IntoView {
    view! {
        <div class="max-w-sm">
            <div>
                <h3 class="text-base font-semibold text-slate-900">"Pith UI"</h3>
                <p class="text-sm text-slate-600">"An open-source component library."</p>
            </div>
            <Separator attr:class=HORIZONTAL decorative=true />
            <div class="flex h-5 items-center gap-4 text-sm text-slate-600">
                <span>"Blog"</span>
                <Separator orientation=Orientation::Vertical attr:class=VERTICAL decorative=true />
                <span>"Docs"</span>
                <Separator orientation=Orientation::Vertical attr:class=VERTICAL decorative=true />
                <span>"Source"</span>
            </div>
        </div>
    }
}
// #endregion demo

// #region demo
use leptos::prelude::*;
use pith_ui::tooltip::*;

const TRIGGER: &str = "rounded-lg border border-slate-200 bg-white px-4 py-2 \
    text-sm font-medium text-slate-700 shadow-sm hover:bg-slate-50 transition-colors";
const TRIGGER_ACCENT: &str = "rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium \
    text-white shadow-sm hover:bg-accent-700 transition-colors";
const CONTENT: &str = "tooltip-content rounded-md bg-slate-900 px-3 py-1.5 text-xs \
    text-white shadow-md";
const ARROW: &str = "fill-slate-900";

#[component]
pub fn TooltipBasic() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger attr:class=TRIGGER>
                        "Hover me"
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent
                            attr:class=CONTENT
                            side_offset=5.0
                        >
                            "This is a tooltip"
                            <TooltipArrow attr:class=ARROW />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger attr:class=TRIGGER_ACCENT>
                        "Save"
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent
                            attr:class=CONTENT
                            side_offset=5.0
                        >
                            "Save your changes"
                            <TooltipArrow attr:class=ARROW />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>
            </TooltipProvider>
        </div>
    }
}
// #endregion demo

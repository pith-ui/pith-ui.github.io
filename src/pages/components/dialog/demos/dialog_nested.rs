// #region demo
use leptos::prelude::*;
use pith_ui::dialog::*;

const BTN: &str = "flex h-10 items-center justify-center rounded-md border \
    border-slate-200 bg-slate-50 px-3.5 text-base font-medium text-slate-900 \
    select-none hover:bg-slate-100 focus-visible:outline-2 \
    focus-visible:-outline-offset-1 focus-visible:outline-blue-800 active:bg-slate-100";
const LINK_BTN: &str = "-mx-1.5 -my-0.5 flex items-center justify-center rounded-xs \
    px-1.5 py-0.5 text-base font-medium text-blue-800 hover:bg-blue-800/5 \
    focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 \
    active:bg-blue-800/10";
const OVERLAY: &str = "dialog-overlay fixed inset-0 bg-black/20";
const CONTENT: &str = "dialog-content fixed left-1/2 top-1/2 -mt-8 w-96 \
    max-w-[calc(100vw-3rem)] -translate-x-1/2 -translate-y-1/2 rounded-lg \
    bg-slate-50 p-6 text-slate-900 outline outline-1 outline-slate-200";
const TITLE: &str = "-mt-1.5 mb-1 text-lg font-medium";
const DESCRIPTION: &str = "mb-6 text-base text-slate-600";

#[component]
pub fn DialogNested() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger attr:class=BTN>
                "View notifications"
            </DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=OVERLAY />
                <DialogContent attr:class=CONTENT>
                    <DialogTitle attr:class=TITLE>
                        "Notifications"
                    </DialogTitle>
                    <DialogDescription attr:class=DESCRIPTION>
                        "You are all caught up. Good job!"
                    </DialogDescription>
                    <div class="flex items-center justify-end gap-4">
                        <div class="mr-auto flex">
                            <Dialog>
                                <DialogTrigger attr:class=LINK_BTN>
                                    "Customize"
                                </DialogTrigger>
                                <DialogPortal>
                                    <DialogOverlay attr:class=OVERLAY />
                                    <DialogContent attr:class=CONTENT>
                                        <DialogTitle attr:class=TITLE>
                                            "Customize notifications"
                                        </DialogTitle>
                                        <DialogDescription attr:class=DESCRIPTION>
                                            "Review your settings here."
                                        </DialogDescription>
                                        <div class="flex items-center justify-end gap-4">
                                            <DialogClose attr:class=BTN>
                                                "Close"
                                            </DialogClose>
                                        </div>
                                    </DialogContent>
                                </DialogPortal>
                            </Dialog>
                        </div>
                        <DialogClose attr:class=BTN>
                            "Close"
                        </DialogClose>
                    </div>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
// #endregion demo

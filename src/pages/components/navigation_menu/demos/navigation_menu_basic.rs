// #region demo
use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavigationMenuBasic() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="relative">
            <NavigationMenuList attr:class="flex items-center gap-1 rounded-lg border border-slate-200 bg-white p-1 shadow-sm">
                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="flex items-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                        "Overview"
                        <svg class="h-2.5 w-2.5 text-slate-500 transition-transform duration-200 [[data-state=open]_&]:rotate-180" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="absolute left-0 top-full mt-1 w-[400px] rounded-lg border border-slate-200 bg-white p-4 shadow-lg">
                        <ul class="grid grid-cols-2 gap-0 list-none p-0 m-0">
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"Quick Start"</div>
                                    <p class="mt-1 text-xs text-slate-500">"Install and assemble your first component."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"Accessibility"</div>
                                    <p class="mt-1 text-xs text-slate-500">"Learn how we build accessible components."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"Releases"</div>
                                    <p class="mt-1 text-xs text-slate-500">"See what\u{2019}s new in the latest versions."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"About"</div>
                                    <p class="mt-1 text-xs text-slate-500">"Learn more about Pith UI and our mission."</p>
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="flex items-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                        "Handbook"
                        <svg class="h-2.5 w-2.5 text-slate-500 transition-transform duration-200 [[data-state=open]_&]:rotate-180" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="absolute left-0 top-full mt-1 rounded-lg border border-slate-200 bg-white p-4 shadow-lg">
                        <ul class="flex max-w-[400px] flex-col list-none p-0 m-0">
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"Styling"</div>
                                    <p class="mt-1 text-xs text-slate-500">"Components can be styled with plain CSS, Tailwind CSS, or CSS modules."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"Animation"</div>
                                    <p class="mt-1 text-xs text-slate-500">"Components can be animated with CSS transitions or animation libraries."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                    <div class="text-sm font-medium text-slate-900">"Composition"</div>
                                    <p class="mt-1 text-xs text-slate-500">"Components can be composed with your own existing components."</p>
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class="rounded-md px-3 py-2 text-sm font-medium text-slate-700 no-underline hover:bg-slate-100 transition-colors" attr:href="https://github.com/pith-ui/pith-ui">
                        "GitHub"
                    </NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
// #endregion demo

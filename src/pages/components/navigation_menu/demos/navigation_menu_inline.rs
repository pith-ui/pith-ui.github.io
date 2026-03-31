// #region demo
use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavigationMenuInline() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="relative">
            <NavigationMenuList attr:class="flex items-center gap-1 rounded-lg border border-slate-200 bg-white p-1 shadow-sm">
                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="flex items-center gap-1.5 rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                        "Product"
                        <svg class="h-2.5 w-2.5 transition-transform duration-200 [[data-state=open]>span>&]:rotate-180" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="absolute left-0 top-full mt-1 rounded-lg border border-slate-200 bg-white shadow-lg overflow-hidden">
                        // Inline nested navigation: tabs on the left, content on the right
                        <NavigationMenu
                            orientation=Orientation::Vertical
                            default_value="developers"
                            attr:class="overflow-hidden text-slate-900"
                        >
                            <div class="grid grid-cols-[12rem_minmax(0,1fr)]">
                                <NavigationMenuList attr:class="flex flex-col gap-0 border-r border-slate-200 bg-slate-50 p-4">
                                    <NavigationMenuItem value="developers">
                                        <NavigationMenuTrigger attr:class="flex w-full flex-col items-start rounded-md p-3 text-left hover:bg-slate-100 data-[state=open]:bg-white data-[state=open]:shadow-sm transition-colors">
                                            <span class="text-sm font-medium text-slate-900">"Developers"</span>
                                            <span class="text-xs text-slate-500">"Build and ship"</span>
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent attr:class="p-6">
                                            <h4 class="m-0 text-base font-medium text-slate-900">"For Developers"</h4>
                                            <p class="m-0 mt-2 text-sm text-slate-500">"Tools and APIs to build great user experiences."</p>
                                            <ul class="-mx-2 mt-4 list-none p-0">
                                                <li>
                                                    <NavigationMenuLink attr:class="block rounded-md p-2 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                                        <div class="text-sm font-medium text-slate-900">"Components"</div>
                                                        <p class="mt-0.5 text-xs text-slate-500">"Pre-built accessible UI primitives."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                                <li>
                                                    <NavigationMenuLink attr:class="block rounded-md p-2 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                                        <div class="text-sm font-medium text-slate-900">"APIs"</div>
                                                        <p class="mt-0.5 text-xs text-slate-500">"Hooks and utilities for custom logic."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                            </ul>
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>
                                    <NavigationMenuItem value="designers">
                                        <NavigationMenuTrigger attr:class="flex w-full flex-col items-start rounded-md p-3 text-left hover:bg-slate-100 data-[state=open]:bg-white data-[state=open]:shadow-sm transition-colors">
                                            <span class="text-sm font-medium text-slate-900">"Designers"</span>
                                            <span class="text-xs text-slate-500">"Design and prototype"</span>
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent attr:class="p-6">
                                            <h4 class="m-0 text-base font-medium text-slate-900">"For Designers"</h4>
                                            <p class="m-0 mt-2 text-sm text-slate-500">"Design kits and tokens for rapid prototyping."</p>
                                            <ul class="-mx-2 mt-4 list-none p-0">
                                                <li>
                                                    <NavigationMenuLink attr:class="block rounded-md p-2 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                                        <div class="text-sm font-medium text-slate-900">"Figma Kit"</div>
                                                        <p class="mt-0.5 text-xs text-slate-500">"A complete design system for Figma."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                                <li>
                                                    <NavigationMenuLink attr:class="block rounded-md p-2 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                                        <div class="text-sm font-medium text-slate-900">"Design Tokens"</div>
                                                        <p class="mt-0.5 text-xs text-slate-500">"Consistent spacing, color, and type scales."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                            </ul>
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>
                                </NavigationMenuList>
                                <NavigationMenuViewport attr:class="relative min-h-[16rem] overflow-hidden" />
                            </div>
                        </NavigationMenu>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class="rounded-md px-3 py-2 text-sm font-medium text-slate-700 no-underline hover:bg-slate-100 transition-colors" attr:href="#">
                        "Releases"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class="rounded-md px-3 py-2 text-sm font-medium text-slate-700 no-underline hover:bg-slate-100 transition-colors" attr:href="#">
                        "GitHub"
                    </NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
// #endregion demo

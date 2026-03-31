// #region demo
use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavigationMenuBasic() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="relative min-w-max rounded-lg bg-slate-50 p-1 text-slate-900">
            <NavigationMenuList attr:class="relative flex">
                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="box-border flex items-center justify-center gap-1.5 h-10 px-3.5 m-0 rounded-md bg-slate-50 text-slate-900 font-medium text-base leading-6 select-none hover:bg-slate-100 active:bg-slate-100 data-[state=open]:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 focus-visible:relative">
                        "Overview"
                        <svg class="transition-transform duration-200 ease-in-out [[data-state=open]_&]:rotate-180" width="10" height="10" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content min-w-[400px] h-full p-6">
                        <ul class="grid list-none grid-cols-[12rem_12rem] gap-0 p-0 m-0">
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"Quick Start"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"Install and assemble your first component."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"Accessibility"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"Learn how we build accessible components."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"Releases"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"See what\u{2019}s new in the latest versions."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"About"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"Learn more about Pith UI and our mission."</p>
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="box-border flex items-center justify-center gap-1.5 h-10 px-3.5 m-0 rounded-md bg-slate-50 text-slate-900 font-medium text-base leading-6 select-none hover:bg-slate-100 active:bg-slate-100 data-[state=open]:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 focus-visible:relative">
                        "Handbook"
                        <svg class="transition-transform duration-200 ease-in-out [[data-state=open]_&]:rotate-180" width="10" height="10" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content min-w-[400px] h-full p-6">
                        <ul class="flex max-w-[400px] flex-col justify-center list-none p-0 m-0">
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"Styling"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"Components can be styled with plain CSS, Tailwind CSS, or CSS modules."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"Animation"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"Components can be animated with CSS transitions, CSS animations, or libraries."</p>
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="block rounded-md p-3 no-underline text-inherit hover:bg-slate-100 focus-visible:relative focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                    <h3 class="m-0 mb-1 text-base leading-5 font-medium">"Composition"</h3>
                                    <p class="m-0 text-sm leading-5 text-slate-500">"Components can be composed with your own existing components."</p>
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class="box-border flex items-center justify-center gap-1.5 h-10 px-3.5 m-0 rounded-md bg-slate-50 text-slate-900 font-medium text-base leading-6 select-none no-underline hover:bg-slate-100 active:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 focus-visible:relative" attr:href="https://github.com/pith-ui/pith-ui">
                        "GitHub"
                    </NavigationMenuLink>
                </NavigationMenuItem>


            </NavigationMenuList>

            <NavigationMenuViewport attr:class="nav-viewport absolute left-0 top-full z-50 mt-2.5 rounded-lg bg-white shadow-lg shadow-slate-200 outline outline-1 outline-slate-200 overflow-hidden" />
        </NavigationMenu>
    }
}
// #endregion demo

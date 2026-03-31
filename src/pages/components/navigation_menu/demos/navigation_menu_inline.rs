// #region demo
use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavigationMenuInline() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="relative min-w-max rounded-lg bg-slate-50 p-1 text-slate-900">
            <NavigationMenuList attr:class="relative flex">
                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="box-border flex items-center justify-center gap-1.5 h-10 px-3.5 m-0 rounded-md bg-slate-50 text-slate-900 font-medium text-base leading-6 select-none hover:bg-slate-100 active:bg-slate-100 data-[state=open]:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 focus-visible:relative">
                        "Product"
                        <svg class="transition-transform duration-200 ease-in-out [[data-state=open]_&]:rotate-180" width="10" height="10" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>

                    <NavigationMenuContent attr:class="nav-content h-full max-w-[675px] p-0">
                        <NavigationMenu
                            attr:class="overflow-hidden text-slate-900"
                            orientation=Orientation::Vertical
                            default_value="developers"
                        >
                            <div class="grid grid-cols-[13rem_minmax(0,1fr)] overflow-clip rounded-lg">
                                <div class="border-r border-r-slate-200 bg-slate-100 overflow-y-auto">
                                <NavigationMenuList attr:class="m-0 flex list-none flex-col gap-0 p-4 h-full">
                                    <NavigationMenuItem value="developers">
                                        <NavigationMenuTrigger attr:class="box-border m-0 flex w-full min-w-[10rem] flex-col items-start gap-0.5 rounded-lg bg-transparent px-3 py-2.5 text-left text-inherit hover:bg-slate-100 data-[state=open]:bg-white data-[state=open]:shadow-[0_1px_2px_rgb(0_0_0_/_0.08),0_1px_1px_rgb(0_0_0_/_0.04)] focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800">
                                            <span class="text-base leading-[1.2] font-medium text-slate-900">"Developers"</span>
                                            <span class="text-sm leading-[1.35] text-slate-500">"Go from idea to UI faster."</span>
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent attr:class="nav-content h-full p-8">
                                            <h4 class="m-0 text-[1.125rem] leading-[1.3] font-medium">"Build product UI without giving up control"</h4>
                                            <p class="m-0 mt-2.5 text-base leading-[1.5] text-slate-500">"Start with accessible parts and shape them to your app instead of working around a preset design system."</p>
                                            <ul class="-mx-2 m-0 mt-4 grid list-none gap-0 p-0">
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Quick start"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Install Pith UI and get your first interactive primitive on screen fast."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Composition"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Wrap and combine parts to match your product structure without hacks."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                            </ul>
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>
                                    <NavigationMenuItem value="systems">
                                        <NavigationMenuTrigger attr:class="box-border m-0 flex w-full min-w-[10rem] flex-col items-start gap-0.5 rounded-lg bg-transparent px-3 py-2.5 text-left text-inherit hover:bg-slate-100 data-[state=open]:bg-white data-[state=open]:shadow-[0_1px_2px_rgb(0_0_0_/_0.08),0_1px_1px_rgb(0_0_0_/_0.04)] focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800">
                                            <span class="text-base leading-[1.2] font-medium text-slate-900">"Design Systems"</span>
                                            <span class="text-sm leading-[1.35] text-slate-500">"Keep patterns aligned across teams."</span>
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent attr:class="nav-content h-full p-8">
                                            <h4 class="m-0 text-[1.125rem] leading-[1.3] font-medium">"Turn shared standards into working components"</h4>
                                            <p class="m-0 mt-2.5 text-base leading-[1.5] text-slate-500">"Connect tokens, states, and accessibility rules once, then give every product team the same solid starting point."</p>
                                            <ul class="-mx-2 m-0 mt-4 grid list-none gap-0 p-0">
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Styling"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Map tokens and component states to your own CSS or utility setup."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Accessibility"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Review keyboard support and semantic defaults before anything ships."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                            </ul>
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>
                                    <NavigationMenuItem value="managers">
                                        <NavigationMenuTrigger attr:class="box-border m-0 flex w-full min-w-[10rem] flex-col items-start gap-0.5 rounded-lg bg-transparent px-3 py-2.5 text-left text-inherit hover:bg-slate-100 data-[state=open]:bg-white data-[state=open]:shadow-[0_1px_2px_rgb(0_0_0_/_0.08),0_1px_1px_rgb(0_0_0_/_0.04)] focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800">
                                            <span class="text-base leading-[1.2] font-medium text-slate-900">"Engineering Leads"</span>
                                            <span class="text-sm leading-[1.35] text-slate-500">"Roll out shared UI without drag."</span>
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent attr:class="nav-content h-full p-8">
                                            <h4 class="m-0 text-[1.125rem] leading-[1.3] font-medium">"Give squads clear defaults and room to move"</h4>
                                            <p class="m-0 mt-2.5 text-base leading-[1.5] text-slate-500">"Use the docs to align on quality bars, upgrades, and extension points while still leaving teams space to customize."</p>
                                            <ul class="-mx-2 m-0 mt-4 grid list-none gap-0 p-0">
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Releases"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Track version changes and migration notes before upgrades surprise teams."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Forms"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Standardize validation and field patterns teams reach for constantly."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                            </ul>
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>
                                    <NavigationMenuItem value="startups">
                                        <NavigationMenuTrigger attr:class="box-border m-0 flex w-full min-w-[10rem] flex-col items-start gap-0.5 rounded-lg bg-transparent px-3 py-2.5 text-left text-inherit hover:bg-slate-100 data-[state=open]:bg-white data-[state=open]:shadow-[0_1px_2px_rgb(0_0_0_/_0.08),0_1px_1px_rgb(0_0_0_/_0.04)] focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800">
                                            <span class="text-base leading-[1.2] font-medium text-slate-900">"Startups"</span>
                                            <span class="text-sm leading-[1.35] text-slate-500">"Ship polished basics while things change."</span>
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent attr:class="nav-content h-full p-8">
                                            <h4 class="m-0 text-[1.125rem] leading-[1.3] font-medium">"Get sturdy UI foundations in place early"</h4>
                                            <p class="m-0 mt-2.5 text-base leading-[1.5] text-slate-500">"Cover the hard interaction details now so your team can spend more time on the product ideas that actually differentiate you."</p>
                                            <ul class="-mx-2 m-0 mt-4 grid list-none gap-0 p-0">
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Quick start"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Get the package installed and your first component working in minutes."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                                <li>
                                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Dialog"</h5>
                                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Launch settings or upgrade flows without rebuilding focus management."</p>
                                                    </NavigationMenuLink>
                                                </li>
                                            </ul>
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>
                                </NavigationMenuList>
                                </div>
                                <NavigationMenuViewport attr:class="nav-viewport relative min-h-[16.5rem] overflow-hidden border-t-0" />
                            </div>
                        </NavigationMenu>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="box-border flex items-center justify-center gap-1.5 h-10 px-3.5 m-0 rounded-md bg-slate-50 text-slate-900 font-medium text-base leading-6 select-none hover:bg-slate-100 active:bg-slate-100 data-[state=open]:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 focus-visible:relative">
                        "Learn"
                        <svg class="transition-transform duration-200 ease-in-out [[data-state=open]_&]:rotate-180" width="10" height="10" viewBox="0 0 10 10" fill="none">
                            <path d="M1 3.5L5 7.5L9 3.5" stroke="currentColor" stroke-width="1.5" />
                        </svg>
                    </NavigationMenuTrigger>

                    <NavigationMenuContent attr:class="nav-content h-full max-w-[500px] p-0">
                        <div class="p-8 text-slate-900">
                            <h4 class="m-0 text-[1.125rem] leading-[1.3] font-medium">"Where teams usually start"</h4>
                            <p class="m-0 mt-2.5 text-base leading-[1.5] text-slate-500">"These are the docs people reach for first when they are turning a prototype into shared UI."</p>
                            <ul class="-mx-2 m-0 mt-4 grid list-none gap-0 p-0">
                                <li>
                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Accessibility handbook"</h5>
                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Take a practical pass over focus order, semantics, and keyboard support."</p>
                                    </NavigationMenuLink>
                                </li>
                                <li>
                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Composition handbook"</h5>
                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Learn when to wrap parts, share behavior, and expose flexible APIs."</p>
                                    </NavigationMenuLink>
                                </li>
                                <li>
                                    <NavigationMenuLink attr:class="box-border block rounded-lg bg-transparent px-2 py-3 no-underline text-inherit hover:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800" attr:href="#">
                                        <h5 class="m-0 text-base leading-[1.25] font-medium">"Styling handbook"</h5>
                                        <p class="m-0 mt-[0.35rem] text-[0.95rem] leading-[1.45] text-slate-500">"Apply tokens and state styles without fighting the underlying markup."</p>
                                    </NavigationMenuLink>
                                </li>
                            </ul>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class="box-border flex items-center justify-center gap-1.5 h-10 px-3.5 m-0 rounded-md bg-slate-50 text-slate-900 font-medium text-base leading-6 select-none no-underline hover:bg-slate-100 active:bg-slate-100 focus-visible:outline-2 focus-visible:-outline-offset-1 focus-visible:outline-blue-800 focus-visible:relative" attr:href="#">
                        "Releases"
                    </NavigationMenuLink>
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

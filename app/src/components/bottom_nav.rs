use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use crate::models::routes::Routes;

const TABS: [(Routes, &str, &str); 3] = [
    (Routes::Home, "/", "Home"),
    (Routes::Blog, "/blog", "Blog"),
    (Routes::Projects, "/projects", "Projects"),
];

#[component]
pub fn BottomNav(is_routing: ReadSignal<bool>) -> impl IntoView {
    let location = use_location();
    let current_tab = Memo::new(move |_| {
        let path = location.pathname.get();

        if path == "/" {
            return Some(Routes::Home);
        }

        TABS.iter()
            .find(|(_, href, _)| {
                if *href == "/" {
                    false
                } else {
                    path.starts_with(href)
                }
            })
            .map(|(tab, _, _)| *tab)
    });

    view! {
        <div class="bg-base-100 fixed bottom-0 left-0 right-0 font-bold border-t border-base-300">
            <div class="grid grid-cols-3 items-center h-8">
                // Left section
                <div class="hidden sm:flex items-center pl-4">
                    <span class="bg-accent px-2">NORMAL</span>
                </div>
                <div class="sm:hidden"></div> // Empty div for mobile

                // Center section
                <div class="flex justify-center items-center">
                    <div class="flex space-x-4 sm:space-x-8">
                        {move || TABS.iter().map(|(tab, href, text)| {
                            let is_active = current_tab().map_or(false, |current| current == *tab);
                            view! {
                                <A href=*href>
                                    <div
                                        class=move || if is_active {
                                            "text-accent transition-colors"
                                        } else {
                                            "hover:text-accent transition-colors"
                                        }>
                                        {text.to_string()}
                                    </div>
                                </A>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Right section
                <div class="flex justify-end pr-4">
                    <div class="relative w-20 sm:w-40">
                        <progress class="progress w-20 sm:w-40 absolute right-0" hidden={move || !is_routing.get()}></progress>
                    </div>
                </div>
            </div>
        </div>
    }
}

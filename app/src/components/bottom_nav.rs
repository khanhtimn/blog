use leptos::prelude::*;
use leptos::either::*;
use leptos_use::{ColorMode, UseColorModeReturn, UseColorModeOptions, use_color_mode_with_options, UseCycleListReturn, UseCycleListOptions, use_cycle_list_with_options};
use leptos_router::hooks::use_location;
use crate::routes::Routes;

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

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .attribute("data-theme")
            .custom_modes(vec![
                "business".to_string(),
                "cupcake".to_string(),
            ])
            .initial_value(ColorMode::Custom("business".into())),
    );

    let UseCycleListReturn { state, next, .. } = use_cycle_list_with_options(
        vec![
            ColorMode::Custom("business".into()),
            ColorMode::Custom("cupcake".into()),
        ],
        UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
    );


    view! {
        <div class="bg-base-100 fixed bottom-0 left-0 right-0 font-bold border-t border-base-300">
            <div class="grid grid-cols-3 items-center h-8">
                <div class="hidden sm:flex items-center pl-4">
                    <button class="btn btn-sm text-sm" on:click=move |_| next()>
                        {move || match state.get() {

                            ColorMode::Custom(ref theme) if theme == "business" => EitherOf3::A(view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 256 256"><path d="M233.54,142.23a8,8,0,0,0-8-2,88.08,88.08,0,0,1-109.8-109.8,8,8,0,0,0-10-10,104.84,104.84,0,0,0-52.91,37A104,104,0,0,0,136,224a103.09,103.09,0,0,0,62.52-20.88,104.84,104.84,0,0,0,37-52.91A8,8,0,0,0,233.54,142.23ZM188.9,190.34A88,88,0,0,1,65.66,67.11a89,89,0,0,1,31.4-26A106,106,0,0,0,96,56,104.11,104.11,0,0,0,200,160a106,106,0,0,0,14.92-1.06A89,89,0,0,1,188.9,190.34Z"></path></svg>
                            }),

                            ColorMode::Custom(ref theme) if theme == "cupcake" => EitherOf3::B(view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="#currentColor" viewBox="0 0 256 256"><path d="M120,40V32a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.07,64.07,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.05,48.05,0,0,0,176,128ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-8-8A8,8,0,0,0,50.34,61.66Zm0,116.68-8,8a8,8,0,0,0,11.32,11.32l8-8a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l8-8a8,8,0,0,0-11.32-11.32l-8,8A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l8,8a8,8,0,0,0,11.32-11.32ZM40,120H32a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Zm88,88a8,8,0,0,0-8,8v8a8,8,0,0,0,16,0v-8A8,8,0,0,0,128,208Zm96-88h-8a8,8,0,0,0,0,16h8a8,8,0,0,0,0-16Z"></path></svg>
                            }),

                            _ => EitherOf3::C(view! { <span>"?"</span> })

                            }}
                        {move || format!("{}", state.get())}
                    </button>
                </div>
                <div class="sm:hidden"></div>

                <div class="flex justify-center items-center">
                    <div class="flex space-x-4 sm:space-x-8">
                        {move || TABS.iter().map(|(tab, href, text)| {
                            let is_active = current_tab() == Some(*tab);
                            view! {
                                <a href=*href>
                                    <div
                                        class=move || if is_active {
                                            "text-accent transition-colors"
                                        } else {
                                            "hover:text-accent transition-colors"
                                        }>
                                        {text.to_string()}
                                    </div>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <div class="flex justify-end pr-4">
                    <div class="relative w-20 sm:w-40">
                        <progress class="progress w-20 sm:w-40 absolute right-0" hidden={move || !is_routing.get()}></progress>
                    </div>
                </div>
            </div>
        </div>
    }
}

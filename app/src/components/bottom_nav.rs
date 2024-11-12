use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::{
    hooks::{use_navigate, use_location},
    NavigateOptions
};
use leptos::ev::{keydown, KeyboardEvent};
use crate::models::routes::Routes;

const TABS: [(Routes, &str, &str); 3] = [
    (Routes::Home, "/", "Home"),
    (Routes::Blog, "/blog", "Blog"),
    (Routes::Projects, "/projects", "Projects"),
];

#[component]
pub fn BottomNav() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    let current_tab = Memo::new(move |_| {
        let path = location.pathname.get();
        TABS.iter()
            .find(|(_, href, _)| *href == path)
            .map(|(tab, _, _)| *tab)
            .unwrap_or(Routes::Home)
    });

    let cycle_tab = move |direction: i32| {
        let current_idx = TABS.iter().position(|(tab, _, _)| *tab == current_tab()).unwrap();
        let new_idx = (current_idx as i32 + direction).rem_euclid(TABS.len() as i32) as usize;
        let (_, new_path, _) = TABS[new_idx];
        navigate(new_path, NavigateOptions::default());
    };

    window_event_listener(keydown, move |e: KeyboardEvent| {
        match e.key().as_str() {
            "h" => cycle_tab(-1),
            "l" => cycle_tab(1),
            _ => (),
        }
    });

    view! {
        <div class="bg-base-100 fixed bottom-0 left-0 right-0 font-bold border-t border-base-300">
            <div class="flex justify-between items-center px-4 h-8">
                <div class="flex items-center space-x-4">
                    <span class="bg-accent px-2">NORMAL</span>
                </div>

                <div class="flex space-x-8">
                    {move || TABS.iter().map(|(tab, href, text)| {
                        let is_active = current_tab() == *tab;
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

                <div class="flex items-center space-x-4">
                    <span>"Top"</span>
                    <span>"1:1"</span>
                    <span class="bg-accent px-2">"02:37"</span>
                </div>
            </div>
        </div>
    }
}

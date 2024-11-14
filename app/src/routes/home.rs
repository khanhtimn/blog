use leptos_router::components::A;
use leptos::prelude::*;
use leptos::ev::{keydown, KeyboardEvent};
use leptos_router::{hooks::use_navigate, NavigateOptions};
use crate::models::routes::Routes;

const NAV_ELEMENTS: [(Routes, &str, &str, &str); 3] = [
    (Routes::Home, "/", "Home Page", "f"),
    (Routes::Blog, "/blog", "Blog Posts", "n"),
    (Routes::Projects, "/projects", "Recent Projects", "r"),
];

#[component]
pub fn HomePage() -> impl IntoView {
    let (selected_idx, set_selected_idx) = signal(0);
    let navigate = use_navigate();

    let cycle_selection = move |direction: i32| {
        let new_idx = (selected_idx.get() as i32 + direction).rem_euclid(NAV_ELEMENTS.len() as i32) as usize;
        set_selected_idx(new_idx);
    };

    let handle_enter = move || {
        let (_, href, _, _) = NAV_ELEMENTS[selected_idx.get()];
        navigate(href, NavigateOptions::default());
    };

    window_event_listener(keydown, move |e: KeyboardEvent| {
        match e.key().as_str() {
            "j" => cycle_selection(1),
            "ArrowDown" => cycle_selection(1),
            "k" => cycle_selection(-1),
            "ArrowUp" => cycle_selection(1),
            "Enter" => handle_enter(),
            _ => (),
        }
    });

    view! {
        <div class="h-screen p-8"
             tabindex="0">
            <div class="border border-dashed border-accent p-6 max-w-2xl mx-auto">
                <h1 class="text-4xl mb-8">Hi, Im Quang Khánh</h1>
                <div class="space-y-2">
                    {NAV_ELEMENTS.iter().enumerate().map(|(idx, (_route, href, text, key))| {
                        view! {
                            <A href=*href>
                                <div
                                    class=move || if selected_idx.get() == idx {
                                        "bg-accent px-2 flex justify-between hover:bg-accent cursor-pointer"
                                    } else {
                                        "flex justify-between hover:bg-accent cursor-pointer"
                                    }
                                    on:click=move |_| set_selected_idx(idx)
                                >
                                    <span>{*text}</span>
                                    <span>{*key}</span>
                                </div>
                            </A>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <div class="mt-8 text-sm">
                <span class="font-bold">Tip:</span> You can use <span class="font-bold">j</span> and <span class="font-bold">k</span> to navigate, and <span class="font-bold">Enter</span> to select. <span class="font-bold">Try it!</span>
                </div>
            </div>
        </div>
    }
}

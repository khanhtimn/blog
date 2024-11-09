use leptos::prelude::*;
use leptos_router::components::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavElements {
    Home,
    Blog,
}

#[component]
#[allow(non_snake_case)]
pub fn Nav() -> impl IntoView {
    let nav_elements = vec![
        (NavElements::Home, "/", "# home"),
        (NavElements::Blog, "/blog", "# view blog posts"),
    ];

    let nav_elements = nav_elements
        .into_iter()
        .map(|(e, href, text)| {
            let exact = e == NavElements::Home;
            view! {
                <A exact=exact href=href><p>{text}</p></A>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="nav">
           <div class="nav-text">
            {nav_elements}
           </div>
        </nav>
    }
}

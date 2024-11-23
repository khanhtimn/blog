pub mod routes;
pub mod components;
pub mod models;
#[cfg(feature = "ssr")]
pub mod state;
#[cfg(feature = "ssr")]
pub mod db;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;
use components::bottom_nav::BottomNav;
use routes::blog_list::BlogList;
use routes::blog_post::BlogPost;
use routes::home::*;
use routes::page_not_found::PageNotFound;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css"/>

        <Title text="Hi, I'm Khánh."/>

        <Router set_is_routing>
            <main>
            <Routes fallback=PageNotFound>
                    <Route
                        path=path!("/")
                        view=HomePage
                    />
                    <Route
                        path=path!("/blog")
                        view=BlogList
                    />
                    <Route
                        path=path!("/blog/:slug")
                        view=BlogPost
                    />
                </Routes>
            </main>
            <BottomNav is_routing=is_routing/>
        </Router>
    }
}

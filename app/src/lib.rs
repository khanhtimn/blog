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
use leptos_router::static_routes::StaticRoute;
use routes::blog_list::BlogList;
use routes::blog_post::BlogPost;

use crate::routes::{nav::*, home::*};

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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css"/>

        <Title text="Welcome to Leptos"/>

        <Router set_is_routing>
            <div class="routing-progress">
                <RoutingProgress is_routing/>
            </div>
            <Nav/>
            <main>
                <Routes fallback=|| "Not found.">
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
        </Router>
    }
}

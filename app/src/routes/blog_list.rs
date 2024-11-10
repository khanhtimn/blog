use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_query_map, use_location};
use leptos::either::Either;
use chrono::Local;

use crate::models::{post::BlogPost, category::Category};
use crate::components::post_card::PostCard;

// This is just example data
// #[server(GetBlogPosts)]
// pub async fn get_blog_posts() -> Result<Vec<BlogPost>, ServerFnError> {
//     Ok(vec![
//         BlogPost {
//             id: 1,
//             title: "Getting Started with Rust and Leptos".to_string(),
//             content: "Rust is a systems programming language...".to_string(),
//             description: "Rust programming language...".to_string(),
//             category: "Rust".to_string(),
//             hero_image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTX6OBtbmbeMEqBGTcdy95TgXxFbZayTrNa6g&s".to_string(),
//             published_at: Local::now(),
//             slug: "getting-started-with-rust-and-leptos".to_string(),
//             },
//         // Add more sample posts...
//     ])
// }

#[server(GetBlogPosts)]
pub async fn get_blog_posts(category_slug: Option<String>) -> Result<Vec<BlogPost>, ServerFnError> {

    use crate::state::AppState;

    let state = expect_context::<AppState>();

    let posts = match category_slug {
        Some(slug) => state.db.search_posts_by_category(&slug).await,
        None => state.db.get_all_posts_with_categories().await,
    };

    posts.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(GetCategories)]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    use crate::state::AppState;

    let state = expect_context::<AppState>();

    state.db.get_all_categories()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn BlogList() -> impl IntoView {
    let query = use_query_map();
    let category = move || query.with(|q| q.get("category"));
    let posts = Resource::new(category, |cat| get_blog_posts(cat));
    let categories = Resource::new(|| (), |_| get_categories());

    let (selected_category, set_selected_category) = signal(None::<String>);

    Effect::new(move |_| {
        let current = query.with(|q| q.get("category"));
        set_selected_category(current);
    });

    view! {
        <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-4">"Latest Articles"</h1>
            //Blog Categories
            <Suspense>
                {move || categories.get().map(|cats| match cats {
                    Ok(categories) => Either::Left(view! {
                        <div class="flex flex-wrap gap-2 mb-8">
                            <A href="/blog">
                                <div
                                    class={move || match selected_category.get() {
                                        None => "px-4 py-2 rounded-full transition-colors bg-blue-600 text-white",
                                        Some(_) => "px-4 py-2 rounded-full transition-colors bg-gray-200 hover:bg-gray-300",
                                    }}
                                >
                                    "All"
                                </div>
                            </A>
                            {categories.into_iter().map(|cat| {
                                let cat_slug = cat.slug.clone();
                                view! {
                                    <A href={format!("/blog?category={}", cat.slug)}>
                                        <div
                                        class={move || match selected_category.get().as_deref() {
                                                Some(current_cat) if current_cat == cat_slug =>
                                                    "px-4 py-2 rounded-full transition-colors bg-blue-600 text-white",
                                                _ => "px-4 py-2 rounded-full transition-colors bg-gray-200 hover:bg-gray-300",
                                            }}
                                        >
                                            {cat.name}
                                        </div>
                                    </A>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }),
                    Err(e) => Either::Right(view! {
                        <div class="text-red-500">"Error loading categories: " {e.to_string()}</div>
                    })
                })}
            </Suspense>

            //Blog Posts
            <Suspense
                fallback=move || view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Skeleton loading cards
                        {(0..3).map(|_| view! {
                            <div class="bg-white rounded-lg shadow-lg overflow-hidden animate-pulse">
                                <div class="h-48 bg-gray-200"></div>
                                <div class="p-6">
                                    <div class="h-4 bg-gray-200 rounded w-1/4 mb-4"></div>
                                    <div class="h-6 bg-gray-200 rounded mb-4"></div>
                                    <div class="h-4 bg-gray-200 rounded mb-2"></div>
                                    <div class="h-4 bg-gray-200 rounded w-3/4"></div>
                                </div>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                }
            >
                {move || match posts.get() {
                    None => view! { <div>"Loading..."</div> }.into_any(),
                    Some(Ok(posts)) => {
                        view! {
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                                {posts.into_iter()
                                    .map(|post| view! { <PostCard post=post/> })
                                    .collect::<Vec<_>>()}
                            </div>
                        }.into_any() }
                    Some(Err(e)) => view! {
                        <div class="text-red-500 p-4 bg-red-50 rounded-lg">
                            "Error loading posts: " {e.to_string()}
                        </div>
                    }.into_any(),
                }}
            </Suspense>
        </div>
    }
}

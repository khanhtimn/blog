use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::models::{post::BlogPost, category::Category};
use crate::components::{post_card::PostCard, post_category::PostCategory};

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
            <h1 class="text-3xl font-bold mb-4">"Latest Articles"</h1>
            //Blog Posts Categories
            <Suspense>
                  {move || match categories.get() {
                      None => view! { <div>"Loading categories..."</div> }.into_any(),
                      Some(Ok(categories)) => view! {
                          <PostCategory
                              categories=categories
                              selected_category=selected_category.get().into()
                          />
                      }.into_any(),
                      Some(Err(e)) => view! {
                          <div class="text-red-500">"Error loading categories: " {e.to_string()}</div>
                      }.into_any(),
                  }}
              </Suspense>

            //Blog Posts Cards
            <Suspense
                fallback=move || view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Skeleton loading cards
                        {(0..3).map(|_| view! {
                            <div class="flex w-52 flex-col gap-4">
                                <div class="skeleton h-32 w-full"></div>
                                <div class="skeleton h-4 w-28"></div>
                                <div class="skeleton h-4 w-full"></div>
                                <div class="skeleton h-4 w-full"></div>
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

use leptos::prelude::*;
use crate::models::post::BlogPost;
use crate::components::post_card::PostCard;
use chrono::Local;

#[server(GetBlogPosts)]
pub async fn get_blog_posts() -> Result<Vec<BlogPost>, ServerFnError> {
    // This is just example data
    Ok(vec![
        BlogPost {
            id: 1,
            title: "Getting Started with Rust and Leptos".to_string(),
            content: "Rust is a systems programming language...".to_string(),
            description: "Rust programming language...".to_string(),
            category: "Rust".to_string(),
            hero_image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTX6OBtbmbeMEqBGTcdy95TgXxFbZayTrNa6g&s".to_string(),
            published_at: Local::now(),
            slug: "getting-started-with-rust-and-leptos".to_string(),
            },
        // Add more sample posts...
    ])
}

#[component]
pub fn BlogList() -> impl IntoView {
    let posts = Resource::new(|| (), |_| get_blog_posts());

    view! {
        <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-4">"Latest Articles"</h1>

            // Categories Filter (optional)
            <div class="flex flex-wrap gap-2 mb-8">
                <button class="px-4 py-2 rounded-full bg-blue-600 text-white">"All"</button>
                <button class="px-4 py-2 rounded-full bg-gray-200 hover:bg-gray-300">"Programming"</button>
                <button class="px-4 py-2 rounded-full bg-gray-200 hover:bg-gray-300">"Design"</button>
            </div>

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
                        }.into_any()
                    }
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

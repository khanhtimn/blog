use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use chrono::Local;
use crate::models::post::BlogPost;

#[server(GetBlogPost)]
pub async fn get_blog_post(slug: String) -> Result<BlogPost, ServerFnError> {
    // This is just example data
    Ok(BlogPost {
        id: 1,
        title: "Getting Started with Rust and Leptos".to_string(),
        content: "Rust is a systems programming language...".to_string(),
        description: "Rust programming language...".to_string(),
        category: "Rust".to_string(),
        hero_image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTX6OBtbmbeMEqBGTcdy95TgXxFbZayTrNa6g&s".to_string(),
        published_at: Local::now(),
        slug,
    })
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").unwrap_or_default());

    let post = Resource::new(slug, |slug| get_blog_post(slug));

    view! {
        <Suspense
            fallback=move || view! {
                <div class="max-w-4xl mx-auto py-12 px-4 sm:px-6 lg:px-8 animate-pulse">
                    <div class="h-8 bg-gray-200 rounded w-3/4 mb-4"></div>
                    <div class="h-4 bg-gray-200 rounded w-1/4 mb-8"></div>
                    <div class="h-96 bg-gray-200 rounded mb-8"></div>
                    <div class="space-y-4">
                        <div class="h-4 bg-gray-200 rounded"></div>
                        <div class="h-4 bg-gray-200 rounded"></div>
                        <div class="h-4 bg-gray-200 rounded w-3/4"></div>
                    </div>
                </div>
            }
        >
            {move || match post.get() {
                None => view! { <div>"Loading..."</div> }.into_any(),
                Some(Ok(post)) => {
                    view! {
                        <article class="max-w-4xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                            // Category Badge
                            <div class="mb-4">
                                <span class="inline-block bg-blue-100 text-blue-800 text-sm px-3 py-1 rounded-full">
                                    {post.category}
                                </span>
                            </div>

                            <h1 class="text-4xl font-bold mb-4">{post.title.clone()}</h1>

                            <div class="text-gray-500 mb-8">
                                {post.published_at.format("%B %d, %Y").to_string()}
                            </div>

                            <p class="text-xl text-gray-600 mb-8">
                                {post.description}
                            </p>

                            // Hero Image
                            <div class="mb-12">
                                <img
                                    src={post.hero_image}
                                    alt={format!("{} cover", post.title)}
                                    class="w-full h-96 object-cover rounded-lg shadow-lg"
                                />
                            </div>

                            // TODO: Implement markdown rendering
                            <div class="prose prose-lg max-w-none">
                                {post.content}
                            </div>
                        </article>
                    }.into_any()
                }
                Some(Err(e)) => view! {
                    <div class="max-w-4xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                        <div class="text-red-500 p-4 bg-red-50 rounded-lg">
                            "Error loading post: " {e.to_string()}
                        </div>
                    </div>
                }.into_any(),
            }}
        </Suspense>
    }
}

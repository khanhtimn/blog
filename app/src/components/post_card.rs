use leptos::prelude::*;
use crate::models::post::BlogPost;

#[component]
pub fn PostCard(post: BlogPost) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow duration-300">
            // Hero Image
            <div class="aspect-w-16 aspect-h-9">
                <img
                    src={post.hero_image}
                    alt={format!("{} cover", post.title.clone())}
                    class="w-full h-48 object-cover"
                />
            </div>
            <div class="p-6">
                // Category Badge
                <div class="mb-4">
                    <span class="inline-block bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full">
                        {post.category}
                    </span>
                </div>

                <h3 class="text-xl font-semibold mb-2">
                    <a href={format!("/blog/{}", post.slug)} class="hover:text-blue-600 transition-colors">
                        {post.title}
                    </a>
                </h3>

                <div class="text-sm text-gray-500 mb-3">
                    {post.published_at.format("%B %d, %Y").to_string()}
                </div>

                <p class="text-gray-600 line-clamp-3">
                    {post.description}
                </p>

                <a
                    href={format!("/blog/{}", post.slug)}
                    class="mt-4 inline-block px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                >
                    "Read More"
                </a>
            </div>
        </div>
    }
}

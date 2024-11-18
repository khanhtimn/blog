use leptos::prelude::*;
use crate::models::post::BlogPost;

#[component]
pub fn PostCard(post: BlogPost) -> impl IntoView {
    view! {
        <div class="card shadow-lg overflow-hidden hover:shadow-xl transition-shadow duration-300">
            <div class="aspect-w-16 aspect-h-9">
                <img
                    src={post.hero_image}
                    alt={format!("{} cover", post.title.clone())}
                    class="w-full h-48 object-cover"
                />
            </div>
            <div class="card-body p-6">

                <h3 class="card-title text-xl font-semibold mb-2">
                    <a href={format!("/blog/{}", post.slug)} class="hover:text-accent transition-colors">
                        {post.title}
                    </a>
                </h3>

                <div class="text-secondary text-sm mb-3">
                    {post.published_at}
                </div>

                <p class="text-primary-content line-clamp-3">
                    {post.description}
                </p>

                <div class="card-actions justify-end">

                    <a href={format!("/blog/{}", post.slug)} >
                        <button class="btn btn-accent text-bold text-l text-base-content">
                            Read more
                        </button>
                    </a>
                </div>
            </div>
        </div>
    }
}

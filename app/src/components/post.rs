use leptos::prelude::*;
use crate::models::post::BlogPost;

#[component]
pub fn Post(post: BlogPost) -> impl IntoView {
    view! {
        <article class="max-w-4xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
            <h1 class="text-4xl font-bold mb-4">{post.title.clone()}</h1>
            <div class="text-gray-500 mb-8">
                {post.published_at}
            </div>
            <p class="text-xl text-gray-600 mb-8">
                {post.description}
            </p>
            <div class="mb-12">
                <img
                    src={post.hero_image}
                    alt={format!("{} cover", post.title)}
                    class="w-full h-96 object-cover rounded-lg shadow-lg"
                />
            </div>
            // <div
            //     class="prose prose-lg max-w-none"
            //     inner_html={post.content}
            // />
            <section class="p-4 mt-4 table-of-contents-parent">
                <h2 class="text-xl md:text-2xl">"Contents"</h2>
                <div
                    class="prose lg:prose-xl dark:prose-invert text-base md: w-full"
                    inner_html={post.toc}
                ></div>
            </section>
            <section
                class="mx-auto prose lg:prose-xl dark:prose-invert text-base mt-8"
                inner_html={post.content}
            ></section>
        </article>
    }
}

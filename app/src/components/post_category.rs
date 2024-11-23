use leptos::prelude::*;
use leptos_router::components::*;

use crate::models::category::Category;

#[component]
pub fn PostCategory(
    categories: Vec<Category>,
    selected_category: ReadSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2 mb-8">
            <A href="/blog">
                <div
                    class={move || match selected_category.get() {
                        None => "text-bold px-4 py-2 rounded-full transition-colors bg-accent",
                        Some(_) => "text-bold px-4 py-2 rounded-full transition-colors hover:bg-accent",
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
                        class={move || match selected_category.get() {
                                Some(current_cat) if current_cat == cat_slug =>
                                    "text-bold px-4 py-2 rounded-full transition-colors bg-accent",
                                _ => "text-bold px-4 py-2 rounded-full transition-colors hover:bg-accent",
                            }}
                        >
                            {cat.name}
                        </div>
                    </A>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

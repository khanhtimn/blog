use leptos::prelude::*;
use leptos::either::*;
use leptos_router::hooks::use_params_map;
use crate::models::post::BlogPost;
use crate::components::post::Post;

#[server(GetBlogPost)]
pub async fn get_blog_post(slug: String) -> Result<BlogPost, ServerFnError> {
    use crate::state::AppState;

    let state = expect_context::<AppState>();

    state.db.get_post_by_slug(&slug)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))

}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").unwrap_or_default());
    let post = Resource::new(slug, get_blog_post);

    view! {
        <Suspense
            fallback=move || view! {
                <div class="flex w-52 flex-col gap-4">
                  <div class="flex items-center gap-4">
                    <div class="skeleton h-16 w-16 shrink-0 rounded-full"></div>
                    <div class="flex flex-col gap-4">
                      <div class="skeleton h-4 w-20"></div>
                      <div class="skeleton h-4 w-28"></div>
                    </div>
                  </div>
                  <div class="skeleton h-32 w-full"></div>
                </div>
            }
        >
            {move || match post.get() {
                None => EitherOf3::A(view! { <div>"Loading..."</div> }),
                Some(Ok(post)) => EitherOf3::B(view! { <Post post /> }),
                Some(Err(e)) => EitherOf3::C(view! {
                    <div class="max-w-4xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                        <div class="text-red-500 p-4 bg-red-50 rounded-lg">
                            "Error loading post: " {e.to_string()}
                        </div>
                    </div>
                }),
            }}
        </Suspense>
    }
}

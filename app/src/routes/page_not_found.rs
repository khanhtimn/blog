use leptos::prelude::*;
use crate::components::flappy_bird::FlappyBird;
use crate::models::flappy_bird::GameAssets;

#[server(GetGameAssets)]
pub async fn get_game_assets() -> Result<GameAssets, ServerFnError> {
    use leptos::logging::log;
    log!("Server: Sending game assets");
    Ok(GameAssets {
        bird_frames: vec![
            "/flappybird/img/bird/b0.png".to_string(),
            "/flappybird/img/bird/b1.png".to_string(),
            "/flappybird/img/bird/b2.png".to_string(),
            "/flappybird/img/bird/b0.png".to_string(),
        ],
        pipe_top: "/flappybird/img/toppipe.png".to_string(),
        pipe_bottom: "/flappybird/img/botpipe.png".to_string(),
        background: "/flappybird/img/BG.png".to_string(),
        ground: "/flappybird/img/ground.png".to_string(),
    })
}

#[component]
pub fn PageNotFound() -> impl IntoView {
    let dialog_ref = NodeRef::<leptos::html::Dialog>::new();
    let (show_game, set_show_game) = signal(false);

    Effect::new(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if show_game.get() {
                dialog.show_modal().unwrap_or_default();
            } else {
                dialog.close();
            }
        }
    });

    view! {
        <div class="min-h-screen bg-base-200 flex flex-col items-center justify-center p-4 relative">
            <div class="card w-96 bg-base-100 shadow-xl">
                <div class="card-body items-center text-center">
                    <h1 class="card-title text-5xl font-bold mb-4">"404"</h1>
                    <p class="text-xl mb-6">"Oops! Page flew away..."</p>

                    <button
                        class="text-4xl mb-4 hover:scale-125 transition-transform cursor-pointer border-none bg-transparent"
                        on:click=move |_| set_show_game.update(|show| *show = !*show)
                    >
                        "🐦"
                    </button>

                    <p class="text-base-content/70 mb-6">
                        "The page you're looking for has taken flight."
                    </p>

                    <div class="card-actions">
                        <a
                            href="/"
                            class="btn btn-accent text-bold text-l text-base-content"
                        >
                            "Go Home"
                        </a>
                    </div>
                </div>
            </div>

            <dialog
                node_ref=dialog_ref
                class="modal"
            >
                <div class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
                    <form method="dialog" class="relative">
                        <button
                            class="btn btn-sm btn-circle btn-ghost absolute -right-8 -top-8 z-50"
                            on:click=move |_| set_show_game.set(false)
                        >
                            "✕"
                        </button>
                    </form>
                    <Await
                        future=get_game_assets()
                        let:assets
                    >
                        <FlappyBird assets={assets.clone().unwrap()}/>
                    </Await>
                </div>
            </dialog>
        </div>
    }
}

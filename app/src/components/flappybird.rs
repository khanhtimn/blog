use leptos::prelude::*;
use leptos_use::{use_interval_fn, use_raf_fn};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

// Game Constants
const CANVAS_WIDTH: u32 = 320;
const CANVAS_HEIGHT: u32 = 480;
const GRAVITY: f64 = 0.125;
const THRUST: f64 = 3.6;
const PIPE_GAP: f64 = 85.0;
const PIPE_WIDTH: f64 = 52.0;
const BIRD_SIZE: f64 = 24.0;
const GROUND_Y: f64 = 400.0;

// Game States
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Loading,
    GetReady,
    Playing,
    GameOver,
}

// Game Types
#[derive(Clone)]
pub struct Bird {
    y: f64,
    speed: f64,
    rotation: f64,
    frame: usize,
}

#[derive(Clone)]
pub struct Pipe {
    x: f64,
    y: f64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GameAssets {
    bird_frames: Vec<String>,
    pipe_top: String,
    pipe_bottom: String,
    background: String,
    ground: String,
}

#[server(GetGameAssets)]
pub async fn get_game_assets() -> Result<GameAssets, ServerFnError> {
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
pub fn FlappyBird() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    // Game state signals
    let (game_state, set_game_state) = signal(GameState::Loading);
    let (score, set_score) = signal(0);
    let (high_score, set_high_score) = signal(0);

    // Asset loading state
    let assets = Resource::new(|| (), |_| async move { get_game_assets().await });
    let (images_loaded, set_images_loaded) = signal(0);
    let total_images = 7;

    // Create refs for the images to track their loading
    let bird_refs = StoredValue::new((0..4).map(|_| NodeRef::<leptos::html::Img>::new()).collect::<Vec<_>>());
    let pipe_top_ref = NodeRef::<leptos::html::Img>::new();
    let pipe_bottom_ref = NodeRef::<leptos::html::Img>::new();
    let bg_ref = NodeRef::<leptos::html::Img>::new();

    Effect::new(move |_| {
        let mut loaded = 0;

        // Check bird frames
        for bird_ref in &bird_refs.get_value() {
            if let Some(img) = bird_ref.get() {
                if img.complete() {
                    loaded += 1;
                }
            }
        }

        // Check other images
        for img_ref in [&pipe_top_ref, &pipe_bottom_ref, &bg_ref] {
            if let Some(img) = img_ref.get() {
                if img.complete() {
                    loaded += 1;
                }
            }
        }

        set_images_loaded.set(loaded);

        if loaded == total_images {
            set_game_state.set(GameState::GetReady);
        }
    });
    // Bird state
    let (bird, set_bird) = signal(Bird {
        y: 100.0,
        speed: 0.0,
        rotation: 0.0,
        frame: 0,
    });

    let (pipes, set_pipes) = signal::<Vec<Pipe>>(vec![]);

    let (frame_count, set_frame_count) = signal(0);

    let _stop = use_raf_fn(move |_| {
        if game_state.get() != GameState::Loading {
            if let Some(canvas) = canvas_ref.get() {
                let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
                update_game(
                    game_state.get(),
                    bird,
                    set_bird,
                    pipes,
                    set_pipes,
                    set_score,
                    frame_count,
                    set_frame_count,
                    set_game_state,
                );
                draw_game(
                    &ctx,
                    game_state.get(),
                    bird.get(),
                    pipes.get(),
                    score.get(),
                    frame_count.get(),
                    assets.get(),
                );
            }
        }
    });

    let _pipe_spawner = use_interval_fn(
        move || {
            if game_state.get() == GameState::Playing {
                spawn_pipe(set_pipes);
            }
        },
        2000,
    );

    let handle_input = move |_| {
        match game_state.get() {
            GameState::GetReady => {
                set_game_state.set(GameState::Playing);
            }
            GameState::Playing => {
                // Bird flap
                set_bird.update(|bird| {
                    bird.speed = -THRUST;
                });
            }
            GameState::GameOver => {
                // Reset game
                set_game_state.set(GameState::GetReady);
                set_bird.set(Bird {
                    y: 100.0,
                    speed: 0.0,
                    rotation: 0.0,
                    frame: 0,
                });
                set_pipes.set(vec![]);
                set_score.set(0);
            }
            GameState::Loading => (),
        }
    };

    view! {
       <div class="game-container">
           <Suspense fallback=move || view! { <div>"Loading assets..."</div> }>
               {move || match assets.get() {
                   Some(Ok(assets)) => view! {
                           <>
                               // Preload images with refs
                               <div class="hidden">
                                   {(0..4).map(|i| view! {
                                       <img
                                           node_ref=bird_refs.get_value()[i]
                                           src={assets.bird_frames[i].clone()}
                                           style="display: none"
                                       />
                                   }).collect_view()}
                                   <img
                                       node_ref=pipe_top_ref
                                       src={assets.pipe_top.clone()}
                                       style="display: none"
                                   />
                                   <img
                                       node_ref=pipe_bottom_ref
                                       src={assets.pipe_bottom.clone()}
                                       style="display: none"
                                   />
                                   <img
                                       node_ref=bg_ref
                                       src={assets.background.clone()}
                                       style="display: none"
                                   />
                               </div>

                               <Show
                                   when=move || images_loaded.get() == total_images
                                   fallback=move || view! {
                                       <div class="loading">
                                           "Loading images... " {images_loaded.get()} "/" {total_images}
                                       </div>
                                   }
                               >
                                   <div class="game-wrapper">
                                       <canvas
                                           node_ref=canvas_ref
                                           width=CANVAS_WIDTH
                                           height=CANVAS_HEIGHT
                                           on:click=move |_| handle_input(())
                                           on:keydown=move |e| {
                                               if e.key() == "Space" {
                                                   handle_input(());
                                               }
                                           }
                                           class="game-canvas"
                                           tabindex="0"
                                       />

                                       <div class="game-ui">
                                           {move || match game_state.get() {
                                               GameState::GetReady => view! {
                                                   <div class="get-ready">
                                                       <div class="message">"Tap to Start"</div>
                                                       <div class="controls">"Space or Click to play"</div>
                                                   </div>
                                               }.into_any(),
                                               GameState::Playing => view! {
                                                   <div class="score-display">
                                                       <div class="current-score">{score.get()}</div>
                                                   </div>
                                               }.into_any(),
                                               GameState::GameOver => view! {
                                                   <div class="game-over">
                                                       <div class="final-score">"Score: " {score.get()}</div>
                                                       <div class="high-score">"Best: " {high_score.get()}</div>
                                                       <div class="restart-prompt">"Tap to Restart"</div>
                                                   </div>
                                               }.into_any(),
                                               GameState::Loading => view! {
                                                   <div class="loading">"Initializing..."</div>
                                               }.into_any()
                                           }}
                                       </div>
                                   </div>
                               </Show>
                           </>
                       }.into_any(),
                   Some(Err(e)) => view! {
                       <div class="error-container">
                           <div class="error-message">
                               "Failed to load game assets"
                           </div>
                           <div class="error-details">
                               {format!("Error: {}", e)}
                           </div>
                       </div>
                   }.into_any(),
                   None => view! { <div class="loading">"Loading..."</div> }.into_any()
               }}
           </Suspense>
       </div>
    }
}

fn spawn_pipe(set_pipes: WriteSignal<Vec<Pipe>>) {
    let mut rng = rand::thread_rng();
    set_pipes.update(|pipes| {
        pipes.push(Pipe {
            x: CANVAS_WIDTH as f64,
            y: -210.0 * (1.0f64.min(rng.gen::<f64>() + 1.0)),
        });
    });
}

fn update_game(
    state: GameState,
    bird: ReadSignal<Bird>,
    set_bird: WriteSignal<Bird>,
    pipes: ReadSignal<Vec<Pipe>>,
    set_pipes: WriteSignal<Vec<Pipe>>,
    set_score: WriteSignal<i32>,
    frame_count: ReadSignal<u32>,
    set_frame_count: WriteSignal<u32>,
    set_game_state: WriteSignal<GameState>,
) {
    set_frame_count.update(|f| *f += 1);

    match state {
        GameState::Playing => {
            // Update bird
            set_bird.update(|bird| {
                bird.speed += GRAVITY;
                bird.y += bird.speed;

                // Update rotation
                if bird.speed <= 0.0 {
                    bird.rotation = (-25.0f64).max(-25.0 * bird.speed / (-1.0 * THRUST));
                } else {
                    bird.rotation = 90.0f64.min(90.0 * bird.speed / (THRUST * 2.0));
                }

                // Update animation frame
                if frame_count.get() % 5 == 0 {
                    bird.frame = (bird.frame + 1) % 4;
                }
            });

            // Update pipes
            set_pipes.update(|pipes| {
                for pipe in pipes.iter_mut() {
                    pipe.x -= 2.0;
                }
                pipes.retain(|pipe| pipe.x > -PIPE_WIDTH);

                // Score update
                if pipes.iter().any(|pipe| pipe.x <= 50.0 && pipe.x > 48.0) {
                    set_score.update(|s| *s += 1);
                }
            });

            // Check collisions
            if check_collision(bird.get(), pipes.get()) {
                set_game_state(GameState::GameOver);
            }
        }
        GameState::GameOver => {
            // Update bird falling
            set_bird.update(|bird| {
                if bird.y < GROUND_Y - BIRD_SIZE {
                    bird.speed += GRAVITY * 2.0;
                    bird.y += bird.speed;
                }
            });
        }
        _ => {}
    }
}

fn draw_game(
    ctx: &web_sys::CanvasRenderingContext2d,
    state: GameState,
    bird: Bird,
    pipes: Vec<Pipe>,
    score: i32,
    _frame_count: u32,
    assets: Option<Result<GameAssets, ServerFnError>>,
){
    if let Some(Ok(assets)) = assets {
        // Clear canvas with sky color
        ctx.set_fill_style(&"#30c0df".into());
        ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        let draw_image = |src: &str, x: f64, y: f64| {
            let img = web_sys::HtmlImageElement::new().unwrap();
            img.set_src(src);
            ctx.draw_image_with_html_image_element(&img, x, y).ok();
        };

        // Draw background
        draw_image(&assets.background, 0.0, CANVAS_HEIGHT as f64 - 512.0);

        // Draw pipes
        for pipe in pipes.iter() {
            // Draw top pipe (inverted)
            draw_image(&assets.pipe_top, pipe.x, pipe.y);
            // Draw bottom pipe at gap distance
            draw_image(
                &assets.pipe_bottom,
                pipe.x,
                pipe.y + PIPE_GAP + 320.0
            );
        }

        ctx.save();
        ctx.translate(50.0, bird.y).unwrap();
        ctx.rotate(bird.rotation * PI / 180.0).unwrap();
        let bird_img = &assets.bird_frames[bird.frame];
        draw_image(bird_img, -BIRD_SIZE/2.0, -BIRD_SIZE/2.0);
        ctx.restore();

        // Draw ground at bottom of canvas
        draw_image(&assets.ground, 0.0, CANVAS_HEIGHT as f64 - 112.0);

        // Draw score
        // if state == GameState::Playing {
        //     ctx.set_fill_style(&"#FFFFFF".into());
        //     let score_text = score.to_string();
        //     let metrics = ctx.measure_text(&score_text).unwrap();
        //     let text_width = metrics.width();
        //     ctx.fill_text(
        //         &score_text,
        //         (CANVAS_WIDTH as f64 - text_width) / 2.0,
        //         50.0
        //     ).ok();
        // }
    }
}

fn check_collision(bird: Bird, pipes: Vec<Pipe>) -> bool {
    if bird.y + BIRD_SIZE > GROUND_Y {
        return true;
    }

    for pipe in pipes {
        if (50.0 + BIRD_SIZE > pipe.x && 50.0 - BIRD_SIZE < pipe.x + PIPE_WIDTH)
            && (bird.y - BIRD_SIZE < pipe.y + 320.0
                || bird.y + BIRD_SIZE > pipe.y + 320.0 + PIPE_GAP)
        {
            return true;
        }
    }
    false
}

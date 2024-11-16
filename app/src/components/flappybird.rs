use leptos::prelude::*;
use leptos::logging::log;
use leptos_use::{use_interval_fn, use_raf_fn, storage::*};
use codee::string::JsonSerdeCodec;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

const CANVAS_WIDTH: u32 = 552;
const CANVAS_HEIGHT: u32 = 480;
const GRAVITY: f64 = 0.125;
const THRUST: f64 = 3.6;
const PIPE_GAP: f64 = 90.0;
const PIPE_TOP_HEIGHT: f64 = 400.0;
const PIPE_Y_MIN_MAX: (f64, f64) = (-350.0, -150.0);
const PIPE_WIDTH: f64 = 52.0;
const BIRD_SIZE: f64 = 34.0;
const GROUND_HEIGHT: f64 = 112.0;
const BACKGROUND_HEIGHT: f64 = 228.0;
const BIRD_X_POSITION: f64 = 80.0;
const BIRD_Y_POSITION: f64 = 80.0;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Loading,
    GetReady,
    Playing,
    GameOver,
}

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
    log!("Server: Getting game assets");
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
    let (game_state, set_game_state) = signal(GameState::Loading);
    let (score, set_score) = signal(0);
    let (frame_count, set_frame_count) = signal(0);

    let (high_score, set_high_score, _) = use_local_storage::<i32, JsonSerdeCodec>(
        "flappy_high_score"
    );

    let (bird, set_bird) = signal(Bird {
        y: BIRD_Y_POSITION,
        speed: 0.0,
        rotation: 0.0,
        frame: 0,
    });

    let (pipes, set_pipes) = signal::<Vec<Pipe>>(vec![]);

    let assets = Resource::new(
        || (),
        move |_| {
            log!("Client: Fetching game assets");
            get_game_assets()
        }
    );

    Effect::new(move |_| {
        match assets.get() {
            Some(Ok(_)) => {
                log!("Assets loaded successfully");
                set_game_state.set(GameState::GetReady);
            }
            Some(Err(e)) => {
                log!("Error loading assets: {:?}", e);
            }
            None => {
                log!("Assets not loaded yet");
            }
        }
    });

    let handle_input = move |_| {
        match game_state.get() {
            GameState::GetReady => {
                set_game_state.set(GameState::Playing);
            }
            GameState::Playing => {
                set_bird.update(|bird| {
                    bird.speed = -THRUST;
                });
            }
            GameState::GameOver => {
               let current_score = score.get();
               if current_score > high_score.get() {
                   set_high_score.set(current_score);
               }

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

    let _pipe_spawner = use_interval_fn(
        move || {
            if game_state.get() == GameState::Playing {
                spawn_pipe(set_pipes);
            }
        },
        2000,
    );

    let game_canvas = move || {
        Suspend::new(async move {
            let game_assets = assets.get_untracked().unwrap();

            let _stop = use_raf_fn(move |_| {
                if game_state.get() != GameState::Loading {
                    if let Some(canvas) = canvas_ref.get() {
                        let ctx = canvas
                            .get_context("2d")
                            .unwrap()
                            .unwrap()
                            .dyn_into::<web_sys::CanvasRenderingContext2d>()
                            .unwrap();

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
                            Some(Ok(game_assets.clone().unwrap())),
                        );
                    }
                }
            });

            view! {
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
            }
        })
    };

    // Main view
    view! {
        <div class="game-container">
            <Suspense fallback=move || view! { <div>"Loading game assets..."</div> }>
                <div class="game-wrapper">
                    {game_canvas}
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
            </Suspense>
        </div>
    }
}


fn spawn_pipe(set_pipes: WriteSignal<Vec<Pipe>>) {
    let mut rng = rand::thread_rng();
    set_pipes.update(|pipes| {
        let y = PIPE_Y_MIN_MAX.0 + (PIPE_Y_MIN_MAX.1 - PIPE_Y_MIN_MAX.0) * rng.gen::<f64>();

        pipes.push(Pipe {
            x: CANVAS_WIDTH as f64,
            y,
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

                if bird.speed <= 0.0 {
                    bird.rotation = (-30.0f64).max(-30.0 * bird.speed / (-1.0 * THRUST));
                } else {
                    bird.rotation = 90.0f64.min(45.0 * bird.speed / THRUST);
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
                if pipes.iter().any(|pipe| pipe.x <= BIRD_X_POSITION && pipe.x > BIRD_X_POSITION - 2.0) {
                    set_score.update(|s| *s += 1);
                }
            });

            // Check collisions with updated positions
            if check_collision(bird.get(), pipes.get()) {
                set_game_state(GameState::GameOver);
            }
        }
        GameState::GameOver => {
            set_bird.update(|bird| {
                if bird.y < (CANVAS_HEIGHT as f64 - GROUND_HEIGHT) - BIRD_SIZE {
                    bird.speed += GRAVITY * 2.0;
                    bird.y += bird.speed;
                    // Keep rotation during fall
                    bird.rotation = 90.0f64.min(bird.rotation + 3.0);
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
    frame_count: u32,
    assets: Option<Result<GameAssets, ServerFnError>>,
){
    if let Some(Ok(assets)) = assets {
        // Clear canvas
        ctx.set_fill_style(&"#30c0df".into());
        ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        let draw_image = |src: &str, x: f64, y: f64| {
            let img = web_sys::HtmlImageElement::new().unwrap();
            img.set_src(src);
            ctx.draw_image_with_html_image_element(&img, x, y).ok();
        };

        draw_image(&assets.background, 0.0, CANVAS_HEIGHT as f64 - BACKGROUND_HEIGHT);
        draw_image(&assets.background, CANVAS_WIDTH as f64 / 2.0, CANVAS_HEIGHT as f64 - BACKGROUND_HEIGHT);
        for pipe in pipes.iter() {
            // Top pipe
            draw_image(&assets.pipe_top, pipe.x, pipe.y);

            // Bottom pipe
            let bottom_pipe_y = pipe.y + PIPE_TOP_HEIGHT + PIPE_GAP;
            draw_image(&assets.pipe_bottom, pipe.x, bottom_pipe_y);
        }

        // Draw bird
        ctx.save();
        ctx.translate(BIRD_X_POSITION, bird.y).unwrap();
        ctx.rotate(bird.rotation * PI / 180.0).unwrap();
        let bird_img = &assets.bird_frames[bird.frame];
        draw_image(bird_img, -BIRD_SIZE/2.0, -BIRD_SIZE/2.0);
        ctx.restore();

        // draw_image(&assets.ground, 0.0, CANVAS_HEIGHT as f64 - GROUND_HEIGHT);
        let ground_offset = match state {
            GameState::Playing | GameState::GetReady => {
                (frame_count as f64 * 2.0) % CANVAS_WIDTH as f64
            },
            _ => 0.0
        };

        draw_image(
            &assets.ground,
            -ground_offset,
            CANVAS_HEIGHT as f64 - GROUND_HEIGHT
        );
        draw_image(
            &assets.ground,
            CANVAS_WIDTH as f64 - ground_offset,
            CANVAS_HEIGHT as f64 - GROUND_HEIGHT
        );
    }
}

fn check_collision(bird: Bird, pipes: Vec<Pipe>) -> bool {
    // Ground collision
    if bird.y + BIRD_SIZE/2.0 > (CANVAS_HEIGHT as f64 - GROUND_HEIGHT) {
        return true;
    }

    // Pipe collision
    for pipe in pipes {
        if BIRD_X_POSITION + BIRD_SIZE/2.0 > pipe.x &&
            BIRD_X_POSITION - BIRD_SIZE/2.0 < pipe.x + PIPE_WIDTH
        {
            let top_pipe_bottom = pipe.y + PIPE_TOP_HEIGHT;
            let bottom_pipe_top = pipe.y + PIPE_TOP_HEIGHT + PIPE_GAP;

            if bird.y - BIRD_SIZE/2.0 < top_pipe_bottom ||
               bird.y + BIRD_SIZE/2.0 > bottom_pipe_top {
                return true;
            }
        }
    }
    false
}

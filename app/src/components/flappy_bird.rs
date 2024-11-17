use leptos::prelude::*;
use leptos::either::*;
use wasm_bindgen::prelude::*;
use leptos::logging::log;
use leptos_use::{use_interval_fn, use_raf_fn, storage::*};
use codee::string::JsonSerdeCodec;
use rand::Rng;
use std::f64::consts::PI;

use crate::models::flappy_bird::*;

#[component]
pub fn FlappyBird(
    assets: Resource<Result<GameAssets, ServerFnError>>,
) -> impl IntoView {
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

    Effect::new(move |_| {
        if game_state.get() != GameState::Loading {
            if let Some(canvas) = canvas_ref.get() {
                let ctx = canvas.get_context("2d").unwrap().unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

                if let Some(Ok(game_assets)) = assets.get() {
                    let _stop = use_raf_fn(move |_| {
                        update_game(
                            game_state.get(),
                            (frame_count, set_frame_count),
                            (bird, set_bird),
                            (pipes, set_pipes),
                            set_score,
                            set_game_state,
                        );

                        draw_game(
                            &ctx,
                            game_state.get(),
                            bird.get(),
                            pipes.get(),
                            frame_count.get(),
                            Some(Ok(game_assets.clone())),
                        );
                    });
                }
            }
        }
    });

    view! {
        <Transition fallback=move || view! { <div>"Loading game assets..."</div> }>
            <div class="relative">
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
                    class="sky-400 border-2 border-base-300 rounded-md shadow-md"
                    tabindex="0"
                />
                <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
                    {move || match game_state.get() {
                        GameState::GetReady => EitherOf4::A(view! {
                            <div class="card overflow-hidden shadow-xl drop-shadow-xl text-white text-center">
                                <div class="card-body p-6">
                                    <div class="card-title font-semibold text-4xl mb-4">"Tap to Start"</div>
                                    <div class="text-xl ">"Space or Click to play"</div>
                                </div>
                            </div>
                        }),
                        GameState::Playing => EitherOf4::B(view! {
                            <div class="absolute top-[50px] text-6xl text-white drop-shadow-lg">
                                <div class="current-score">{score.get()}</div>
                            </div>
                        }),
                        GameState::GameOver => EitherOf4::C(view! {
                            <div class="card overflow-hidden shadow-xl drop-shadow-xl text-white text-center">
                                <div class="card-body p-6">
                                    <div class="card-title font-semibold text-4xl mb-4">"Score: " {score.get()}</div>
                                    <div class="card-title font-semibold text-4xl mb-4">"Best: " {high_score.get()}</div>
                                    <div class="text-4xl mb-4">"Tap to Restart"</div>
                                </div>
                            </div>
                        }),
                        GameState::Loading => EitherOf4::D(view! {
                            <div class="loading">"Initializing..."</div>
                        })
                    }}
                </div>
            </div>
        </Transition>
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
    (frame_count, set_frame_count): (ReadSignal<u32>, WriteSignal<u32>),
    (bird, set_bird): (ReadSignal<Bird>, WriteSignal<Bird>),
    (pipes, set_pipes): (ReadSignal<Vec<Pipe>>, WriteSignal<Vec<Pipe>>),
    set_score: WriteSignal<i32>,
    set_game_state: WriteSignal<GameState>,
) {
    set_frame_count.update(|f| *f += 1);

    match state {
        GameState::Playing => {
            set_bird.update(|bird| {
                bird.speed += GRAVITY;
                bird.y += bird.speed;

                if bird.speed <= 0.0 {
                    bird.rotation = (-30.0f64).max(-30.0 * bird.speed / (-1.0 * THRUST));
                } else {
                    bird.rotation = 90.0f64.min(45.0 * bird.speed / THRUST);
                }

                if frame_count.get() % 5 == 0 {
                    bird.frame = (bird.frame + 1) % 4;
                }
            });

            set_pipes.update(|pipes| {
                for pipe in pipes.iter_mut() {
                    pipe.x -= 2.0;
                }
                pipes.retain(|pipe| pipe.x > -PIPE_WIDTH);

                if pipes.iter().any(|pipe| pipe.x <= BIRD_X_POSITION && pipe.x > BIRD_X_POSITION - 2.0) {
                    set_score.update(|s| *s += 1);
                }
            });

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

#[allow(deprecated)]
fn draw_game(
    ctx: &web_sys::CanvasRenderingContext2d,
    state: GameState,
    bird: Bird,
    pipes: Vec<Pipe>,
    frame_count: u32,
    assets: Option<Result<GameAssets, ServerFnError>>,
){
    if let Some(Ok(assets)) = assets {
        ctx.set_fill_style(&BACKDROP_COLOR.into());
        ctx.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        let draw_image = |src: &str, x: f64, y: f64| {
            let img = web_sys::HtmlImageElement::new().unwrap();
            img.set_src(src);
            ctx.draw_image_with_html_image_element(&img, x, y).ok();
        };

        draw_image(&assets.background, 0.0, CANVAS_HEIGHT as f64 - BACKGROUND_HEIGHT);
        draw_image(&assets.background, CANVAS_WIDTH as f64 / 2.0, CANVAS_HEIGHT as f64 - BACKGROUND_HEIGHT);
        for pipe in pipes.iter() {
            draw_image(&assets.pipe_top, pipe.x, pipe.y);

            let bottom_pipe_y = pipe.y + PIPE_TOP_HEIGHT + PIPE_GAP;
            draw_image(&assets.pipe_bottom, pipe.x, bottom_pipe_y);
        }

        ctx.save();
        ctx.translate(BIRD_X_POSITION, bird.y).unwrap();
        ctx.rotate(bird.rotation * PI / 180.0).unwrap();
        let bird_img = &assets.bird_frames[bird.frame];
        draw_image(bird_img, -BIRD_SIZE/2.0, -BIRD_SIZE/2.0);
        ctx.restore();

        let ground_offset = match state {
            GameState::Playing => {
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
    if bird.y + BIRD_SIZE/2.0 > (CANVAS_HEIGHT as f64 - GROUND_HEIGHT) {
        return true;
    }

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

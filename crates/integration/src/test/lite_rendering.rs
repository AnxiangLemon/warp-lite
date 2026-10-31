use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use instant::{Duration, Instant};
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::vec2f;
use settings::Setting as _;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use warp::integration_testing::step::new_step_with_default_assertions;
use warp::integration_testing::terminal::util::ExpectedExitStatus;
use warp::integration_testing::terminal::{
    execute_command_for_single_terminal_in_tab, wait_until_bootstrapped_single_pane_for_tab,
};
use warp::settings::RenderingDeviceSetting;
use warpui_core::r#async::Timer;
use warpui_core::integration::{AssertionOutcome, TestStep};
use warpui_core::rendering::FrameRateLimit;

use super::backgrounds::{PNG_A, assert_background_image, prepare_background};
use super::lite_builder;
use crate::Builder;

pub fn test_lite_software_rendering() -> Builder {
    software_rendering_test(false)
}

pub fn test_lite_software_rendering_background() -> Builder {
    software_rendering_test(true)
}

fn software_rendering_test(background: bool) -> Builder {
    let mut builder = lite_builder()
        .with_real_display()
        .with_user_defaults(HashMap::from([(
            RenderingDeviceSetting::storage_key().to_owned(),
            "\"Software\"".to_owned(),
        )]))
        .with_setup(move |utils| {
            prepare_background(utils, background.then_some(PNG_A));
            utils.set_env("WARP_FORCE_SOFTWARE_RENDERING", None::<&str>);
            utils.set_env("WGPU_BACKEND", Some("dx12"));
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0));
    if background {
        builder = builder.with_step(
            new_step_with_default_assertions("Background is loaded before pacing measurements")
                .add_named_assertion("Cached background bitmap is ready", |app, _| {
                    assert_background_image(app)
                }),
        );
    }
    for limit in [
        FrameRateLimit::Automatic,
        FrameRateLimit::Fps15,
        FrameRateLimit::Unlimited,
    ] {
        builder = builder.with_step(measure_redraws(limit, true));
    }
    builder = builder.with_step(measure_redraws(FrameRateLimit::Fps15, false));
    builder.with_step(execute_command_for_single_terminal_in_tab(
        0,
        "Write-Output 'software-rendering-responsive'".to_owned(),
        ExpectedExitStatus::Success,
        "software-rendering-responsive".to_owned(),
    ))
}

fn cpu_time_ms() -> u64 {
    let pid = Pid::from_u32(std::process::id());
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::nothing().with_cpu(),
    );
    system
        .process(pid)
        .expect("test process exists")
        .accumulated_cpu_time()
}

fn measure_redraws(limit: FrameRateLimit, resize: bool) -> TestStep {
    let result = Rc::new(RefCell::new(None));
    let result_for_action = result.clone();
    new_step_with_default_assertions("Software rendering coalesces interactive redraws")
        .set_timeout(Duration::from_secs(45))
        .with_action(move |app, window_id, _| {
            let mut app = app.clone();
            let result = result_for_action.clone();
            let frames = Rc::new(Cell::new(0));
            let frame_counter = frames.clone();
            app.update(|ctx| {
                ctx.update_rendering_config(|config| config.frame_rate_limit = limit);
                assert!(
                    ctx.windows()
                        .platform_window(window_id)
                        .unwrap()
                        .uses_software_rendering()
                );
                ctx.on_frame_drawn(move |_, drawn_window| {
                    if drawn_window == window_id {
                        frame_counter.set(frame_counter.get() + 1);
                    }
                });
            });
            let executor = app.read(|ctx| ctx.foreground_executor().clone());
            executor
                .spawn(async move {
                    let initial_cpu = cpu_time_ms();
                    let start = Instant::now();
                    for tick in 0..120 {
                        app.update(|ctx| {
                            if resize && tick % 10 == 0 {
                                ctx.windows().set_window_bounds(
                                    window_id,
                                    RectF::new(
                                        vec2f(100., 100.),
                                        vec2f(1000. + (tick % 30) as f32, 650.),
                                    ),
                                );
                            }
                            ctx.windows()
                                .platform_window(window_id)
                                .unwrap()
                                .as_ctx()
                                .request_redraw();
                        });
                        Timer::after(Duration::from_millis(4)).await;
                    }
                    app.update(|ctx| {
                        ctx.on_next_frame_drawn(window_id, move || {
                            *result.borrow_mut() = Some((
                            frames.get(),
                            start.elapsed(),
                            cpu_time_ms() - initial_cpu,
                        ));
                        });
                        ctx.windows()
                            .platform_window(window_id)
                            .unwrap()
                            .as_ctx()
                            .request_redraw();
                    });
                })
                .detach();
        })
        .add_named_assertion(
            "The final frame is presented within the software frame budget",
            move |_, _| {
                let Some((frames, elapsed, cpu_ms)) = *result.borrow() else {
                    return AssertionOutcome::failure(
                        "Waiting for the final software frame".to_owned(),
                    );
                };
                let maximum = match limit {
                    FrameRateLimit::Automatic | FrameRateLimit::Fps30 => 30.,
                    FrameRateLimit::Fps15 => 15.,
                    FrameRateLimit::Fps60 => 60.,
                    FrameRateLimit::Unlimited => f64::INFINITY,
                };
                let ceiling = (elapsed.as_secs_f64() * maximum).ceil() as usize;
                if frames < 2 || frames > ceiling.saturating_add(2) {
                    return AssertionOutcome::failure(format!(
                        "Expected 2..={ceiling} frames, got {frames} in {elapsed:?}"
                    ));
                }
                println!(
                    "Software rendering {limit:?} resize={resize}: {frames} frames in {:.3}s ({:.1} FPS), cpu_ms={cpu_ms}",
                    elapsed.as_secs_f64(),
                    frames as f64 / elapsed.as_secs_f64()
                );
                AssertionOutcome::Success
            },
        )
        .with_take_screenshot(format!("lite-software-rendering-{limit:?}-resize-{resize}.png"))
}

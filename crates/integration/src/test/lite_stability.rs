use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use instant::{Duration, Instant};
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::vec2f;
use settings::Setting as _;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use warp::appearance::Appearance;
use warp::integration_testing::pane_group::{assert_num_panes_in_tab, close_pane_by_index};
use warp::integration_testing::terminal::{
    assert_no_block_executing, wait_until_bootstrapped_pane,
};
use warp::integration_testing::view_getters::{terminal_view, workspace_view};
use warp::integration_testing::workspace::assert_tab_count;
use warp::settings::{FrameRateLimitSetting, GPUSettings, RenderingDeviceSetting};
use warp::terminal::TerminalView;
use warp::workspace::WorkspaceAction;
use warpui_core::assets::asset_cache::{AssetCache, AssetState};
use warpui_core::r#async::Timer;
use warpui_core::image_cache::ImageType;
use warpui_core::integration::{AssertionOutcome, TestStep};
use warpui_core::{SingletonEntity as _, async_assert};

use super::backgrounds::{PNG_A, prepare_background};
use super::{lite_builder, test_state_dir};
use crate::Builder;

pub fn test_lite_multiterminal_stability() -> Builder {
    stability_test(false)
}

pub fn test_lite_multiterminal_background_stability() -> Builder {
    stability_test(true)
}

fn stability_test(background: bool) -> Builder {
    let automatic = std::env::var_os("WARP_TEST_STABILITY_AUTOMATIC").is_some();
    let adaptive = automatic || std::env::var_os("WARP_TEST_STABILITY_ADAPTIVE").is_some();
    let mut builder = lite_builder()
        .with_real_display()
        .with_timeout(Duration::from_secs(600))
        .with_user_defaults(HashMap::from([
            (
                RenderingDeviceSetting::storage_key().to_owned(),
                if automatic {
                    "\"Automatic\""
                } else {
                    "\"Software\""
                }
                .to_owned(),
            ),
            (
                FrameRateLimitSetting::storage_key().to_owned(),
                if adaptive {
                    "\"Automatic\""
                } else {
                    "\"Fps15\""
                }
                .to_owned(),
            ),
        ]))
        .with_setup(move |utils| {
            let fixture = if background {
                std::env::var_os("WARP_TEST_STABILITY_BACKGROUND")
                    .map(|path| std::fs::read(path).expect("read stress background fixture"))
                    .unwrap_or_else(|| PNG_A.to_vec())
            } else {
                Vec::new()
            };
            prepare_background(utils, background.then_some(fixture.as_slice()));
            utils.set_env("WARP_FORCE_SOFTWARE_RENDERING", None::<&str>);
            utils.set_env("WGPU_BACKEND", Some("dx12"));
        })
        .with_step(wait_until_bootstrapped_pane(0, 0))
        .with_step(
            TestStep::new("Check the selected renderer and loaded background").add_named_assertion(
                "The requested rendering workload is active",
                move |app, window_id| {
                    app.read(|ctx| {
                        let software = ctx
                            .windows()
                            .platform_window(window_id)
                            .unwrap()
                            .uses_software_rendering();
                        assert!(automatic || software);
                        println!(
                            "STABILITY_RENDERER software={software}, frame_rate={:?}",
                            GPUSettings::as_ref(ctx).frame_rate_limit.value()
                        );
                        let image = Appearance::as_ref(ctx).theme().background_image();
                        if !background {
                            return async_assert!(image.is_none(), "Expected a solid background");
                        }
                        let Some(image) = image else {
                            return AssertionOutcome::failure("Waiting for background".to_owned());
                        };
                        let AssetState::Loaded { data } =
                            AssetCache::as_ref(ctx).load_asset::<ImageType>(image.source)
                        else {
                            return AssertionOutcome::failure(
                                "Waiting for background pixels".to_owned(),
                            );
                        };
                        let ImageType::StaticBitmap { image } = data.as_ref() else {
                            panic!("Expected a static background");
                        };
                        println!("STABILITY_BACKGROUND {}x{}", image.width(), image.height());
                        async_assert!(
                            image.width() <= 2048 && image.height() <= 2048,
                            "Background texture must be bounded"
                        )
                    })
                },
            ),
        )
        .with_step(measure_phase("baseline".to_owned(), 10, false));
    for cycle in 0..3 {
        for tab in 0..4 {
            if tab > 0 {
                builder = builder
                    .with_step(
                        TestStep::new("Add an independent terminal tab")
                            .with_action(|app, window_id, _| {
                                let workspace = workspace_view(app, window_id);
                                app.dispatch_typed_action(
                                    window_id,
                                    &[workspace.id()],
                                    &WorkspaceAction::AddTerminalTab {
                                        hide_homepage: true,
                                    },
                                );
                            })
                            .add_named_assertion("Tab added", assert_tab_count(tab + 1)),
                    )
                    .with_step(wait_until_bootstrapped_pane(tab, 0));
            }
            builder = builder
                .with_step(
                    TestStep::new("Split the active tab")
                        .with_keystrokes(&["ctrl-shift-D"])
                        .add_named_assertion("Two panes exist", assert_num_panes_in_tab(tab, 2)),
                )
                .with_step(wait_until_bootstrapped_pane(tab, 1));
        }
        builder = builder
            .with_step(start_workers(cycle))
            .with_step(measure_phase(
                format!("cycle-{cycle}-interactive"),
                20,
                true,
            ))
            .with_step(wait_for_workers(cycle))
            .with_step(measure_phase(format!("cycle-{cycle}-eight-idle"), 5, false));
        if cycle == 2 {
            builder = builder.with_step(fault_isolation());
        }
        for tab in (1..4).rev() {
            builder = builder.with_step(
                TestStep::new("Close a completed terminal tab")
                    .with_action(move |app, window_id, _| {
                        let workspace = workspace_view(app, window_id);
                        app.dispatch_typed_action(
                            window_id,
                            &[workspace.id()],
                            &WorkspaceAction::ActivateTab(tab),
                        );
                        app.dispatch_typed_action(
                            window_id,
                            &[workspace.id()],
                            &WorkspaceAction::CloseActiveTab,
                        );
                    })
                    .add_named_assertion("Tab and panes were removed", assert_tab_count(tab)),
            );
        }
        builder = builder
            .with_step(close_pane_by_index(0, 1))
            .with_step(measure_phase(
                format!("cycle-{cycle}-after-close"),
                10,
                false,
            ));
    }
    builder.with_step(measure_phase("after-undo-expiry".to_owned(), 65, false)).with_step(TestStep::new("Original terminal remains usable after all cycles")
        .with_action(|app, window_id, _| {
            terminal_view(app, window_id, 0, 0).update(app, |view, ctx| {
                view.execute_command_or_set_pending("[IO.File]::WriteAllText((Join-Path $env:WARP_LITE_TEST_STATE_DIR 'final-responsive'), 'ok')", ctx);
            });
        })
        .add_named_assertion("The surviving shell executes a new command", |_, _| {
            async_assert!(test_state_dir().join("final-responsive").is_file(), "Waiting for the surviving shell")
        })
        .add_named_assertion("Closed terminal views are released after undo expiry", |app, window_id| {
            let count = app.views_of_type::<TerminalView>(window_id).unwrap_or_default().len();
            async_assert!(count == 1, "Expected only the surviving terminal view, got {count}")
        })
        .add_named_assertion("Shell returned to its prompt", assert_no_block_executing(0, 0)))
}

fn start_workers(cycle: usize) -> TestStep {
    TestStep::new("Run eight concurrent PTY output streams")
        .with_action(move |app, window_id, _| {
            for tab in 0..4 {
                for pane in 0..2 {
                    let command = format!(
                        "$sw=[Diagnostics.Stopwatch]::StartNew(); $n=0; while($sw.Elapsed.TotalSeconds -lt 22) {{ Write-Output ('worker-{tab}-{pane}-'+$n+' '+('x'*100)); $n++; Start-Sleep -Milliseconds 50 }}; for($n=0;$n -lt 5000;$n++) {{ Write-Output ('burst-'+$n+' '+('y'*100)) }}; [IO.File]::WriteAllText((Join-Path $env:WARP_LITE_TEST_STATE_DIR 'done-{cycle}-{tab}-{pane}'), 'ok')"
                    );
                    terminal_view(app, window_id, tab, pane).update(app, |view, ctx| {
                        view.execute_command_or_set_pending(&command, ctx);
                    });
                }
            }
        })
}

fn wait_for_workers(cycle: usize) -> TestStep {
    let mut step = TestStep::new("All eight output tasks finish and return to their prompts")
        .set_timeout(Duration::from_secs(90))
        .add_named_assertion("Every worker completed its bounded output", move |_, _| {
            async_assert!(
                (0..4).all(|tab| (0..2).all(|pane| test_state_dir()
                    .join(format!("done-{cycle}-{tab}-{pane}"))
                    .is_file())),
                "Waiting for all eight workers"
            )
        });
    for tab in 0..4 {
        for pane in 0..2 {
            step = step.add_named_assertion(
                "Worker returned to its prompt",
                assert_no_block_executing(tab, pane),
            );
        }
    }
    step
}

fn fault_isolation() -> TestStep {
    let mut step = TestStep::new("A failed task and force-terminated shell leave seven peers usable")
        .set_timeout(Duration::from_secs(45))
        .with_action(|app, window_id, _| {
            for tab in 0..4 {
                for pane in 0..2 {
                    let command = if (tab, pane) == (3, 1) {
                        "cmd /d /c exit 23; [IO.File]::WriteAllText((Join-Path $env:WARP_LITE_TEST_STATE_DIR 'task-exit-code'), [string]$LASTEXITCODE); Stop-Process -Id $PID -Force".to_owned()
                    } else {
                        format!("Start-Sleep -Seconds 3; Write-Output 'survivor-{tab}-{pane}'; [IO.File]::WriteAllText((Join-Path $env:WARP_LITE_TEST_STATE_DIR 'survivor-{tab}-{pane}'), 'ok')")
                    };
                    terminal_view(app, window_id, tab, pane).update(app, |view, ctx| {
                        view.execute_command_or_set_pending(&command, ctx);
                    });
                }
            }
        })
        .add_named_assertion("The shell observed the failing task exit code", |_, _| {
            async_assert!(std::fs::read_to_string(test_state_dir().join("task-exit-code")).ok().as_deref() == Some("23"), "Expected task exit code 23")
        })
        .add_named_assertion("All seven other shells execute after the failure", |_, _| {
            async_assert!((0..4).all(|tab| (0..2).all(|pane| (tab, pane) == (3, 1) || test_state_dir().join(format!("survivor-{tab}-{pane}")).is_file())), "Waiting for surviving shells")
        })
        .add_named_assertion("The terminated shell pane becomes read-only", |app, window_id| {
            terminal_view(app, window_id, 3, 1).read(app, |view, _| {
                async_assert!(view.model.lock().is_read_only(), "Waiting for the terminated shell state")
            })
        })
        .add_named_assertion("The exited pane remains available for its output", assert_num_panes_in_tab(3, 2))
        .add_named_assertion("All tabs remain available", assert_tab_count(4));
    for tab in 0..4 {
        for pane in 0..2 {
            if (tab, pane) != (3, 1) {
                step = step.add_named_assertion(
                    "Surviving shell returned to its prompt",
                    assert_no_block_executing(tab, pane),
                );
            }
        }
    }
    step
}

fn process_sample() -> (u64, u64, u64) {
    let pid = Pid::from_u32(std::process::id());
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::nothing().with_cpu().with_memory(),
    );
    let process = system.process(pid).expect("test GUI process exists");
    (
        process.accumulated_cpu_time(),
        process.memory(),
        process.virtual_memory(),
    )
}

fn measure_phase(name: String, seconds: u64, interactive: bool) -> TestStep {
    let result = Rc::new(RefCell::new(None));
    let result_for_action = result.clone();
    TestStep::new("Measure GUI resources under the current terminal workload")
        .set_timeout(Duration::from_secs(seconds + 45))
        .with_action(move |app, window_id, _| {
            let mut app = app.clone();
            let result = result_for_action.clone();
            let name = name.clone();
            let frames = Rc::new(Cell::new(0));
            let frame_counter = frames.clone();
            app.update(|ctx| {
                ctx.on_frame_drawn(move |_, drawn_window| {
                    if drawn_window == window_id {
                        frame_counter.set(frame_counter.get() + 1);
                    }
                })
            });
            let executor = app.read(|ctx| ctx.foreground_executor().clone());
            executor
                .spawn(async move {
                    let initial = process_sample();
                    let start = Instant::now();
                    let mut peak_rss = initial.1;
                    let mut peak_private = initial.2;
                    let mut tick = 0;
                    let mut longest_tick_ms = 0;
                    println!("STABILITY_PHASE_START {name}");
                    while start.elapsed() < Duration::from_secs(seconds) {
                        let tick_start = Instant::now();
                        if interactive {
                            let workspace = workspace_view(&app, window_id);
                            app.dispatch_typed_action(
                                window_id,
                                &[workspace.id()],
                                &WorkspaceAction::ActivateTab(tick % 4),
                            );
                            if tick % 4 == 0 {
                                app.update(|ctx| {
                                    ctx.windows().set_window_bounds(
                                        window_id,
                                        RectF::new(
                                            vec2f(100., 100.),
                                            vec2f(1000. + (tick % 12) as f32 * 10., 650.),
                                        ),
                                    )
                                });
                            }
                        }
                        Timer::after(Duration::from_millis(250)).await;
                        let sample = process_sample();
                        peak_rss = peak_rss.max(sample.1);
                        peak_private = peak_private.max(sample.2);
                        assert!(
                            peak_private < 2 * 1024 * 1024 * 1024,
                            "Bounded stress exceeded the 2 GiB GUI memory guard"
                        );
                        longest_tick_ms = longest_tick_ms.max(tick_start.elapsed().as_millis());
                        tick += 1;
                    }
                    let final_sample = process_sample();
                    let metrics = serde_json::json!({
                        "phase": name, "elapsed_seconds": start.elapsed().as_secs_f64(),
                        "cpu_ms": final_sample.0 - initial.0, "frames": frames.get(),
                        "rss_bytes": final_sample.1, "private_bytes": final_sample.2,
                        "peak_rss_bytes": peak_rss, "peak_private_bytes": peak_private,
                        "longest_tick_ms": longest_tick_ms,
                    });
                    println!("STABILITY_METRICS {metrics}");
                    *result.borrow_mut() = Some(metrics);
                })
                .detach();
        })
        .add_named_assertion(
            "The bounded measurement completes without freezing",
            move |_, _| {
                async_assert!(
                    result.borrow().is_some(),
                    "Collecting GUI resource measurements"
                )
            },
        )
}

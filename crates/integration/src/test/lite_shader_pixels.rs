use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{Vector2F, vec2f};
use settings::Setting as _;
use warp::integration_testing::terminal::wait_until_bootstrapped_single_pane_for_tab;
use warp::settings::RenderingDeviceSetting;
use warpui_core::assets::asset_cache::Asset;
use warpui_core::color::ColorU;
use warpui_core::elements::{
    AfterLayoutContext, Element, Empty, EventContext, Fill, LayoutContext, PaintContext, Point,
    SizeConstraint,
};
use warpui_core::event::DispatchedEvent;
use warpui_core::image_cache::{CustomImageFormat, CustomImageHeader, ImageType, StaticImage};
use warpui_core::integration::{AssertionOutcome, TestStep};
use warpui_core::platform::{CapturedFrame, WindowStyle};
use warpui_core::scene::{Border, CornerRadius, Radius};
use warpui_core::{AppContext, Entity, Scene, TypedActionView, View, WindowId};

use super::backgrounds::prepare_background;
use super::lite_builder;
use crate::Builder;

pub fn test_lite_software_shader_pixels() -> Builder {
    let automatic = std::env::var_os("WARP_TEST_STABILITY_AUTOMATIC").is_some();
    let window_id: Rc<Cell<Option<WindowId>>> = Rc::new(Cell::new(None));
    let new_window = window_id.clone();
    let ready_window = window_id.clone();
    let capture: Arc<Mutex<Option<CapturedFrame>>> = Arc::new(Mutex::new(None));
    let capture_for_action = capture.clone();
    lite_builder()
        .with_real_display()
        .with_user_defaults(HashMap::from([(
            RenderingDeviceSetting::storage_key().to_owned(),
            if automatic {
                "\"Automatic\""
            } else {
                "\"Software\""
            }
            .to_owned(),
        )]))
        .with_setup(|utils| {
            prepare_background(utils, None);
            utils.set_env("WGPU_BACKEND", Some("dx12"));
        })
        .with_step(wait_until_bootstrapped_single_pane_for_tab(0))
        .with_step(
            TestStep::new("Open a deterministic shader reference view")
                .with_action(move |app, _, _| {
                    let (id, _) = app.add_window(WindowStyle::Normal, |_| ShaderPixelsView {
                        image: fixture_image(),
                    });
                    new_window.set(Some(id));
                })
                .add_named_assertion(
                    "The reference window has a rendering device",
                    move |app, _| {
                        let ready = ready_window.get().is_some_and(|id| {
                            app.read(|ctx| ctx.windows().platform_window(id).is_some())
                        });
                        if ready {
                            AssertionOutcome::Success
                        } else {
                            AssertionOutcome::failure("Waiting for shader window".to_owned())
                        }
                    },
                ),
        )
        .with_step(
            TestStep::new("Read back software shader reference pixels")
                .with_action(move |app, _, _| {
                    app.read(|ctx| {
                        let window = ctx
                            .windows()
                            .platform_window(window_id.get().unwrap())
                            .unwrap();
                        assert_eq!(window.uses_software_rendering(), !automatic);
                        println!(
                            "SHADER_RENDERER software={}",
                            window.uses_software_rendering()
                        );
                        let capture = capture_for_action.clone();
                        window
                            .as_ctx()
                            .request_frame_capture(Box::new(move |mut frame| {
                                frame.ensure_rgba();
                                *capture.lock().unwrap() = Some(frame);
                            }));
                    });
                })
                .add_named_assertion(
                    "Gradient, alpha order, borders, corners and bilinear images match",
                    move |_, _| {
                        let captured = capture.lock().unwrap();
                        let Some(frame) = captured.as_ref() else {
                            return AssertionOutcome::failure(
                                "Waiting for shader pixels".to_owned(),
                            );
                        };
                        for (x, y, expected) in [
                            (15, 15, [128u8, 0, 0, 255]),
                            (25, 25, [64, 0, 128, 255]),
                            (50, 15, [188, 0, 67, 255]),
                            (50, 25, [94, 0, 161, 255]),
                            (90, 10, [0, 0, 0, 255]),
                            (104, 10, [0, 0, 255, 255]),
                            (104, 25, [255, 255, 0, 255]),
                            (10, 50, [76, 0, 0, 255]),
                            (20, 55, [67, 4, 2, 255]),
                            (29, 59, [24, 21, 22, 255]),
                            (70, 55, [225, 12, 6, 255]),
                            (99, 50, [0, 0, 0, 255]),
                        ] {
                            let offset = ((y * frame.width + x) * 4) as usize;
                            let actual = &frame.data[offset..offset + 4];
                            if !actual
                                .iter()
                                .zip(expected)
                                .all(|(&actual, expected)| actual.abs_diff(expected) <= 2)
                            {
                                return AssertionOutcome::failure(format!(
                                    "Pixel ({x}, {y}): expected {expected:?}, got {actual:?}"
                                ));
                            }
                        }
                        println!(
                            "SHADER_PIXELS gradient/alpha/order/border/corner/bilinear passed"
                        );
                        AssertionOutcome::Success
                    },
                ),
        )
}

fn fixture_image() -> Arc<StaticImage> {
    let pixels = vec![
        255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 128, 255, 255, 255, 255,
    ];
    let bytes =
        CustomImageHeader::prepend_custom_header(pixels, 2, 2, CustomImageFormat::Rgba).unwrap();
    let ImageType::StaticBitmap { image } = ImageType::try_from_bytes(&bytes).unwrap() else {
        panic!("expected bitmap")
    };
    image
}

struct ShaderPixelsView {
    image: Arc<StaticImage>,
}
impl Entity for ShaderPixelsView {
    type Event = ();
}
impl TypedActionView for ShaderPixelsView {
    type Action = ();
}
impl View for ShaderPixelsView {
    fn ui_name() -> &'static str {
        "ShaderPixels"
    }
    fn render(&self, _: &AppContext) -> Box<dyn Element> {
        ShaderPixelsElement {
            empty: Empty::new(),
            image: self.image.clone(),
        }
        .finish()
    }
}

struct ShaderPixelsElement {
    empty: Empty,
    image: Arc<StaticImage>,
}
impl Element for ShaderPixelsElement {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        self.empty.layout(constraint, ctx, app)
    }
    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &AppContext) {
        self.empty.after_layout(ctx, app);
    }
    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        self.empty.paint(origin, ctx, app);
        paint_fixture(ctx.scene, &self.image);
    }
    fn size(&self) -> Option<Vector2F> {
        self.empty.size()
    }
    fn origin(&self) -> Option<Point> {
        self.empty.origin()
    }
    fn dispatch_event(
        &mut self,
        event: &DispatchedEvent,
        ctx: &mut EventContext,
        app: &AppContext,
    ) -> bool {
        self.empty.dispatch_event(event, ctx, app)
    }
}

fn paint_fixture(scene: &mut Scene, image: &Arc<StaticImage>) {
    let scale = scene.scale_factor();
    let rect = |x, y, width, height| RectF::new(vec2f(x, y) / scale, vec2f(width, height) / scale);
    scene
        .draw_rect_without_hit_recording(rect(0., 0., 128., 96.))
        .background = Fill::Solid(ColorU::new(0, 0, 0, 255));
    scene
        .draw_rect_without_hit_recording(rect(10., 10., 20., 20.))
        .background = Fill::Solid(ColorU::new(255, 0, 0, 128));
    scene
        .draw_rect_without_hit_recording(rect(40., 10., 40., 20.))
        .background = Fill::Gradient {
        start: vec2f(0., 0.),
        end: vec2f(1., 0.),
        start_color: ColorU::new(255, 0, 0, 255),
        end_color: ColorU::new(0, 0, 255, 255),
    };
    scene
        .draw_rect_without_hit_recording(rect(20., 20., 40., 10.))
        .background = Fill::Solid(ColorU::new(0, 0, 255, 128));
    let rounded = scene.draw_rect_without_hit_recording(rect(90., 10., 30., 30.));
    rounded.background = Fill::Solid(ColorU::new(255, 255, 0, 255));
    rounded.corner_radius = CornerRadius::with_all(Radius::Pixels(8. / scale));
    rounded.border = Border {
        width: 2. / scale,
        color: Fill::Solid(ColorU::new(0, 0, 255, 255)),
        top: true,
        right: true,
        bottom: true,
        left: true,
        dash: None,
    };
    scene.draw_image(
        rect(10., 50., 40., 20.),
        image.clone(),
        0.3,
        CornerRadius::default(),
        true,
    );
    scene.draw_image(
        rect(60., 50., 40., 20.),
        image.clone(),
        1.,
        CornerRadius::default(),
        false,
    );
}

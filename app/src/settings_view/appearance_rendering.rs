use settings::Setting as _;
use warpui::elements::{Container, Element, Flex, ParentElement};
use warpui::platform::GraphicsBackend;
use warpui::rendering::{FrameRateLimit, RenderingDevice};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::{AppContext, SingletonEntity, ViewContext};

use super::{AppearancePageAction, AppearanceSettingsPageView};
use crate::appearance::Appearance;
use crate::settings::{GPUSettings, PreferredGraphicsBackend};
use crate::settings_view::settings_page::{
    LocalOnlyIconState, SettingsWidget, render_dropdown_item,
};
use crate::view_components::DropdownItem;

impl AppearanceSettingsPageView {
    pub(super) fn refresh_rendering_dropdowns(&mut self, ctx: &mut ViewContext<Self>) {
        if !cfg!(windows) || (!cfg!(feature = "warp_lite") && cfg!(feature = "agent_runtime")) {
            return;
        }
        self.rendering_device_dropdown.update(ctx, |dropdown, ctx| {
            dropdown.set_items(
                [RenderingDevice::Automatic, RenderingDevice::Software]
                    .into_iter()
                    .map(|device| {
                        DropdownItem::new(
                            rendering_device_label(device),
                            AppearancePageAction::SetRenderingDevice(device),
                        )
                    })
                    .collect(),
                ctx,
            );
            dropdown.set_selected_by_name(
                rendering_device_label(*GPUSettings::as_ref(ctx).rendering_device.value()),
                ctx,
            );
        });
        self.frame_rate_limit_dropdown.update(ctx, |dropdown, ctx| {
            dropdown.set_items(
                [
                    FrameRateLimit::Automatic,
                    FrameRateLimit::Fps15,
                    FrameRateLimit::Fps30,
                    FrameRateLimit::Fps60,
                    FrameRateLimit::Unlimited,
                ]
                .into_iter()
                .map(|limit| {
                    DropdownItem::new(
                        frame_rate_limit_label(limit),
                        AppearancePageAction::SetFrameRateLimit(limit),
                    )
                })
                .collect(),
                ctx,
            );
            dropdown.set_selected_by_name(
                frame_rate_limit_label(*GPUSettings::as_ref(ctx).frame_rate_limit.value()),
                ctx,
            );
        });
        self.graphics_backend_dropdown.update(ctx, |dropdown, ctx| {
            if let Some(window) = ctx.windows().platform_window(ctx.window_id()) {
                let mut items = vec![DropdownItem::new(
                    crate::settings_view::lite_i18n::text("Default"),
                    AppearancePageAction::SetPreferredGraphicsBackend(None),
                )];
                // Lite keeps driver initialization off the settings path; unavailable preferences
                // fall back through the existing adapter selection on the next window.
                let backends = if cfg!(feature = "warp_lite") {
                    vec![
                        GraphicsBackend::Dx12,
                        GraphicsBackend::Vulkan,
                        GraphicsBackend::Gl,
                    ]
                } else {
                    window.supported_backends()
                };
                items.extend(backends.into_iter().map(|backend| {
                    DropdownItem::new(
                        backend.to_label(),
                        AppearancePageAction::SetPreferredGraphicsBackend(Some(backend)),
                    )
                }));
                dropdown.set_items(items, ctx);
            }
            let gpu_settings = GPUSettings::as_ref(ctx);
            dropdown.set_selected_by_name(
                gpu_settings
                    .preferred_backend
                    .map(|backend| backend.to_label())
                    .unwrap_or(crate::settings_view::lite_i18n::text("Default")),
                ctx,
            );
        });
    }
}

fn rendering_device_label(device: RenderingDevice) -> &'static str {
    match device {
        RenderingDevice::Automatic => "自动（GPU 优先）",
        RenderingDevice::Software => "CPU 软件渲染",
    }
}

fn frame_rate_limit_label(limit: FrameRateLimit) -> &'static str {
    match limit {
        FrameRateLimit::Automatic => "自动（自适应）",
        FrameRateLimit::Fps15 => "最高 15 FPS",
        FrameRateLimit::Fps30 => "最高 30 FPS",
        FrameRateLimit::Fps60 => "最高 60 FPS",
        FrameRateLimit::Unlimited => "不限（按需绘制）",
    }
}

pub(super) struct RenderingDeviceWidget;

impl SettingsWidget for RenderingDeviceWidget {
    type View = AppearanceSettingsPageView;

    fn search_terms(&self) -> &str {
        "gpu cpu software hardware rendering device 渲染 设备 软件 硬件 显卡 服务器"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let dropdown = render_dropdown_item(
            appearance,
            "渲染设备",
            None,
            None,
            LocalOnlyIconState::Hidden,
            None,
            &view.rendering_device_dropdown,
        );
        let current = app
            .windows()
            .platform_window(view.window_id)
            .map(|window| {
                if window.uses_software_rendering() {
                    "当前窗口：CPU 软件渲染。"
                } else {
                    "当前窗口：硬件或驱动提供的渲染设备。"
                }
            })
            .unwrap_or("当前设备尚未初始化。");
        Flex::column().with_child(dropdown)
            .with_child(appearance.ui_builder().wrappable_text(current, true).build().finish())
            .with_child(appearance.ui_builder().wrappable_text(
                "自动优先使用可用 GPU，没有 GPU 时回退到 CPU。软件渲染由 CPU 绘图，适合无 GPU 或驱动异常的环境；背景图片和大窗口会增加开销。新建窗口或重启后生效。", true).build().finish())
            .finish()
    }
}

pub(super) struct FrameRateLimitWidget;

impl SettingsWidget for FrameRateLimitWidget {
    type View = AppearanceSettingsPageView;

    fn search_terms(&self) -> &str {
        "fps frame rate limit refresh performance background 帧率 刷新 上限 性能 背景 卡顿"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let dropdown = render_dropdown_item(
            appearance,
            "帧率上限",
            None,
            None,
            LocalOnlyIconState::Hidden,
            None,
            &view.frame_rate_limit_dropdown,
        );
        let limit = *GPUSettings::as_ref(app).frame_rate_limit.value();
        let description = match limit {
            FrameRateLimit::Automatic => {
                "自动：CPU 软件渲染最高 30 FPS，绘制繁忙时进一步降频；GPU 不额外限帧。"
            }
            FrameRateLimit::Fps15 => {
                "最高 15 FPS：降低绘制开销，适合 CPU 资源紧张的服务器；拖动和滚动会更不连贯。"
            }
            FrameRateLimit::Fps30 => "最高 30 FPS：在操作流畅度和 CPU 开销之间折中。",
            FrameRateLimit::Fps60 => "最高 60 FPS：操作更连贯，需要更多渲染资源。",
            FrameRateLimit::Unlimited => {
                "不限：不增加应用层帧率限制，仍受渲染耗时和显示同步约束；软件渲染可能占满 CPU。"
            }
        };
        Flex::column().with_child(dropdown)
            .with_child(appearance.ui_builder().wrappable_text(description, true).build().finish())
            .with_child(appearance.ui_builder().wrappable_text("修改立即生效。这是帧率上限，不是固定帧率；仅在内容变化或系统请求重绘时绘制。", true).build().finish())
            .finish()
    }
}

#[derive(Default)]
pub(super) struct GraphicsBackendWidget {}

impl SettingsWidget for GraphicsBackendWidget {
    type View = AppearanceSettingsPageView;

    fn search_terms(&self) -> &str {
        "graphics rendering backend vulkan dx12 directx12 opengl driver 图形 渲染 后端 驱动"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        let dropdown = render_dropdown_item(
            appearance,
            crate::settings_view::lite_i18n::text("Preferred graphics backend"),
            None,
            None,
            LocalOnlyIconState::for_setting(
                PreferredGraphicsBackend::storage_key(),
                PreferredGraphicsBackend::sync_to_cloud(),
                &mut view.local_only_icon_tooltip_states.borrow_mut(),
                app,
            ),
            None,
            &view.graphics_backend_dropdown,
        );
        let mut col = Flex::column().with_child(dropdown);
        if let Some(window) = app.windows().platform_window(view.window_id) {
            let backend = window.graphics_backend();
            col.add_child(
                appearance
                    .ui_builder()
                    .wrappable_text(
                        format!(
                            "{}: {}",
                            crate::settings_view::lite_i18n::text("Current backend"),
                            backend.to_label()
                        ),
                        true,
                    )
                    .with_style(UiComponentStyles {
                        font_color: Some(theme.sub_text_color(theme.background()).into_solid()),
                        ..Default::default()
                    })
                    .build()
                    .finish(),
            );
            if cfg!(feature = "warp_lite") {
                col.add_child(appearance.ui_builder().wrappable_text(
                    "默认：自动选择。DirectX 12：Windows 首选，支持 GPU 和 CPU 软件渲染。Vulkan：需兼容驱动。OpenGL：兼容性备选，并不保证更省 CPU。不可用时自动回退；新建窗口或重启后生效。", true).build().finish());
            }
        }
        if view.graphics_backend_preference_changed {
            col.add_child(
                Container::new(
                    appearance
                        .ui_builder()
                        .wrappable_text(
                            crate::settings_view::lite_i18n::text(
                                "Changes will apply to new windows.",
                            ),
                            true,
                        )
                        .with_style(UiComponentStyles {
                            font_color: Some(theme.sub_text_color(theme.background()).into_solid()),
                            ..Default::default()
                        })
                        .build()
                        .finish(),
                )
                .with_margin_bottom(10.)
                .finish(),
            );
        }
        col.finish()
    }
}

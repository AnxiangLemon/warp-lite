use warpui::assets::asset_cache::AssetSource;
use warpui::elements::{
    Align, CacheOption, ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, Image,
    MainAxisAlignment, MouseStateHandle, ParentElement, Text, Wrap,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::components::UiComponent;
use warpui::{AppContext, Entity, View, ViewContext, ViewHandle};

use super::SettingsSection;
use super::settings_page::{
    MatchData, PageType, SettingsPageEvent, SettingsPageMeta, SettingsPageViewHandle,
    SettingsWidget,
};
use crate::appearance::Appearance;
use crate::channel::ChannelState;
use crate::themes::theme::ColorScheme;
use crate::workspace::WorkspaceAction;

pub struct AboutPageView {
    page: PageType<Self>,
}

impl AboutPageView {
    pub fn new(_ctx: &mut ViewContext<AboutPageView>) -> Self {
        AboutPageView {
            page: PageType::new_monolith(
                AboutPageWidget::default(),
                cfg!(feature = "warp_lite").then_some("关于"),
                false,
            ),
        }
    }
}

impl Entity for AboutPageView {
    type Event = SettingsPageEvent;
}

impl View for AboutPageView {
    fn ui_name() -> &'static str {
        "AboutPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

#[derive(Default)]
struct AboutPageWidget {
    copy_version_button_mouse_state: MouseStateHandle,
}

impl SettingsWidget for AboutPageWidget {
    type View = AboutPageView;

    fn search_terms(&self) -> &str {
        "about warp version"
    }

    fn render(
        &self,
        _view: &AboutPageView,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        if cfg!(feature = "warp_lite") {
            return self.render_lite(appearance);
        }
        let theme = appearance.theme();
        let ui_builder = appearance.ui_builder();

        let image_path = if theme.inferred_color_scheme() == ColorScheme::LightOnDark {
            "bundled/svg/warp-logo-with-light-title.svg"
        } else {
            "bundled/svg/warp-logo-with-dark-title.svg"
        };

        let version = if cfg!(feature = "warp_lite") {
            concat!("Warp Lite ", env!("CARGO_PKG_VERSION"))
        } else {
            ChannelState::app_version().unwrap_or("v#.##.###")
        };

        let version_text = ui_builder
            .span(version.to_string())
            .with_soft_wrap()
            .build()
            .with_margin_top(16.)
            .finish();

        let copy_version_icon = appearance
            .ui_builder()
            .copy_button(16., self.copy_version_button_mouse_state.clone())
            .build()
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::CopyVersion(version));
            })
            .finish();

        let version_row = Wrap::row()
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_children([
                version_text,
                Container::new(copy_version_icon)
                    .with_margin_top(16.)
                    .with_padding_left(6.)
                    .finish(),
            ]);

        Align::new(
            Flex::column()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(
                    ConstrainedBox::new(
                        Image::new(
                            AssetSource::Bundled { path: image_path },
                            CacheOption::BySize,
                        )
                        .finish(),
                    )
                    .with_max_height(100.)
                    .with_max_width(350.)
                    .finish(),
                )
                .with_child(version_row.finish())
                .with_child(
                    ui_builder
                        .span(super::lite_i18n::text("Copyright 2026 Warp"))
                        .build()
                        .with_margin_top(16.)
                        .finish(),
                )
                .finish(),
        )
        .finish()
    }
}

impl AboutPageWidget {
    fn render_lite(&self, appearance: &Appearance) -> Box<dyn Element> {
        let theme = appearance.theme();
        let font = appearance.ui_font_family();
        let foreground = theme.foreground().into_solid();
        let secondary = theme.sub_text_color(theme.surface_1()).into_solid();
        let version = concat!("Warp Lite ", env!("CARGO_PKG_VERSION"));
        let logo = ConstrainedBox::new(
            Image::new(
                AssetSource::Bundled {
                    path: "bundled/svg/warp-lite-logo.svg",
                },
                CacheOption::BySize,
            )
            .finish(),
        )
        .with_width(88.)
        .with_height(88.)
        .finish();
        let version_row = Flex::row()
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(
                Text::new(format!("版本 {}", env!("CARGO_PKG_VERSION")), font, 13.)
                    .with_color(secondary)
                    .finish(),
            )
            .with_child(
                Container::new(
                    appearance
                        .ui_builder()
                        .copy_button(16., self.copy_version_button_mouse_state.clone())
                        .build()
                        .on_click(move |ctx, _, _| {
                            ctx.dispatch_typed_action(WorkspaceAction::CopyVersion(version))
                        })
                        .finish(),
                )
                .with_margin_left(8.)
                .finish(),
            )
            .finish();
        Align::new(
            ConstrainedBox::new(
                Flex::column()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_child(
                        Container::new(logo)
                            .with_margin_top(24.)
                            .with_margin_bottom(20.)
                            .finish(),
                    )
                    .with_child(
                        Text::new("Warp Lite", font, 28.)
                            .with_style(Properties {
                                weight: Weight::Bold,
                                ..Default::default()
                            })
                            .with_color(foreground)
                            .finish(),
                    )
                    .with_child(
                        Container::new(
                            Text::new("轻量终端，专注当下。", font, 15.)
                                .with_color(secondary)
                                .finish(),
                        )
                        .with_margin_top(10.)
                        .with_margin_bottom(24.)
                        .finish(),
                    )
                    .with_child(version_row)
                    .with_child(
                        Container::new(
                            Text::new("本地终端  ·  分屏管理  ·  会话恢复", font, 13.)
                                .with_color(foreground)
                                .finish(),
                        )
                        .with_margin_top(28.)
                        .with_margin_bottom(20.)
                        .finish(),
                    )
                    .with_child(
                        Text::new(
                            "基于 Warp 裁剪构建的精简版本，保留日常终端工作所需的核心功能。",
                            font,
                            14.,
                        )
                        .with_color(secondary)
                        .finish(),
                    )
                    .with_child(
                        Container::new(
                            Text::new(
                                "衍生版本：Warp Lite\n原项目：Warp · 保留原项目版权与许可声明",
                                font,
                                12.,
                            )
                            .with_color(secondary)
                            .finish(),
                        )
                        .with_margin_top(28.)
                        .with_margin_bottom(24.)
                        .finish(),
                    )
                    .finish(),
            )
            .with_max_width(440.)
            .finish(),
        )
        .finish()
    }
}

impl SettingsPageMeta for AboutPageView {
    fn section() -> SettingsSection {
        SettingsSection::About
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<AboutPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<AboutPageView>) -> Self {
        SettingsPageViewHandle::About(view_handle)
    }
}

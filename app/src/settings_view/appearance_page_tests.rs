use warpui::App;

use super::AppearanceSettingsPageView;
#[cfg(windows)]
use super::rendering::{FrameRateLimitWidget, GraphicsBackendWidget, RenderingDeviceWidget};
use crate::settings_view::settings_page::FilteredPageType;
#[cfg(windows)]
use crate::settings_view::settings_page::SettingsWidget;

#[test]
fn lite_appearance_search_keeps_theme_title_and_omits_removed_background_controls() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = AppearanceSettingsPageView::build_lite_page();
            let FilteredPageType::Categorized {
                categories, title, ..
            } = page.get_filtered()
            else {
                panic!("expected categorized appearance page");
            };
            let full_count = categories
                .iter()
                .map(|category| category.widgets.len())
                .sum::<usize>();
            assert_eq!(title, Some("外观"));
            assert!(
                !categories
                    .iter()
                    .any(|category| category.title == "背景图片")
            );

            assert!(page.update_filter("主题", ctx).is_truthy());
            let FilteredPageType::Categorized {
                categories, title, ..
            } = page.get_filtered()
            else {
                panic!("expected categorized appearance page");
            };
            assert_eq!(title, Some("外观"));
            assert_eq!(categories.len(), 1);
            assert_eq!(categories[0].title, "主题");
            assert_eq!(categories[0].widgets.len(), 1);

            for query in ["背景图片", "wallpaper"] {
                assert!(!page.update_filter(query, ctx).is_truthy(), "{query}");
            }
            page.update_filter("", ctx);
            let FilteredPageType::Categorized {
                categories, title, ..
            } = page.get_filtered()
            else {
                panic!("expected categorized appearance page");
            };
            assert_eq!(title, Some("外观"));
            assert_eq!(
                categories
                    .iter()
                    .map(|category| category.widgets.len())
                    .sum::<usize>(),
                full_count
            );
        });
    });
}

#[cfg(windows)]
#[test]
fn rendering_search_isolates_each_control_and_clears() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = AppearanceSettingsPageView::build_lite_page();
            let FilteredPageType::Categorized { categories, .. } = page.get_filtered() else {
                panic!("expected categorized appearance page");
            };
            assert_eq!(categories.last().unwrap().title, "图形渲染");
            assert_eq!(categories.last().unwrap().widgets.len(), 3);
            let full_count = categories
                .iter()
                .map(|category| category.widgets.len())
                .sum::<usize>();
            for (query, expected) in [
                ("CPU", RenderingDeviceWidget::static_widget_id()),
                ("渲染 设备", RenderingDeviceWidget::static_widget_id()),
                ("Vulkan", GraphicsBackendWidget::static_widget_id()),
                ("图形 后端", GraphicsBackendWidget::static_widget_id()),
                ("FPS", FrameRateLimitWidget::static_widget_id()),
                ("背景", FrameRateLimitWidget::static_widget_id()),
            ] {
                page.update_filter(query, ctx);
                let FilteredPageType::Categorized {
                    categories, title, ..
                } = page.get_filtered()
                else {
                    panic!("expected categorized appearance page");
                };
                assert_eq!(categories.len(), 1, "query: {query}");
                assert_eq!(categories[0].title, "图形渲染");
                assert_eq!(categories[0].widgets.len(), 1);
                assert_eq!(categories[0].widgets[0].widget_id(), expected);
                assert_eq!(title, Some("外观"));
            }
            page.update_filter("", ctx);
            let FilteredPageType::Categorized { categories, .. } = page.get_filtered() else {
                panic!("expected categorized appearance page");
            };
            assert_eq!(
                categories
                    .iter()
                    .map(|category| category.widgets.len())
                    .sum::<usize>(),
                full_count
            );
        });
    });
}

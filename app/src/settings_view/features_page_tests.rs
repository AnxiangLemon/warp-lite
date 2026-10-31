use warpui::App;

use super::super::settings_page::FilteredPageType;
use super::*;

#[test]
fn rendering_search_isolates_each_control_and_clears() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = PageType::new_uncategorized(
                vec![
                    Box::new(RenderingDeviceWidget),
                    Box::new(GraphicsBackendWidget::default()),
                    Box::new(FrameRateLimitWidget),
                    Box::new(MouseScrollMultiplierWidget::default()),
                ],
                Some("终端"),
            );
            for (query, expected) in [
                ("CPU", RenderingDeviceWidget::static_widget_id()),
                ("渲染 设备", RenderingDeviceWidget::static_widget_id()),
                ("Vulkan", GraphicsBackendWidget::static_widget_id()),
                ("图形 后端", GraphicsBackendWidget::static_widget_id()),
                ("FPS", FrameRateLimitWidget::static_widget_id()),
                ("背景", FrameRateLimitWidget::static_widget_id()),
            ] {
                page.update_filter(query, ctx);
                let FilteredPageType::Uncategorized { widgets, title, .. } = page.get_filtered()
                else {
                    panic!("expected an uncategorized settings page");
                };
                assert_eq!(widgets.len(), 1, "query: {query}");
                assert_eq!(widgets[0].widget_id(), expected);
                assert_eq!(title, Some("终端"));
            }
            page.update_filter("", ctx);
            let FilteredPageType::Uncategorized { widgets, .. } = page.get_filtered() else {
                panic!("expected an uncategorized settings page");
            };
            assert_eq!(widgets.len(), 4);
        });
    });
}

//! Lite stub for agent-mode input rendering.

use warpui::elements::{Element, Empty};
use warpui::{AppContext, ViewContext, ViewHandle};

use super::Input;
use crate::terminal::view::ambient_agent::HostSelector;

impl Input {
    pub fn is_cloud_mode_input_v2_composing(&self, _app: &AppContext) -> bool {
        false
    }

    pub(super) fn render_agent_input(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub(super) fn should_show_auth_secret_ftux(&self, _app: &AppContext) -> bool {
        false
    }

    pub(super) fn visible_host_selector(
        &self,
        _app: &AppContext,
    ) -> Option<&ViewHandle<HostSelector>> {
        None
    }

    pub(super) fn render_ambient_agent_status_footer(
        &self,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

pub mod styles {
    use pathfinder_color::ColorU;
    use warp_core::ui::theme::WarpTheme;

    use crate::ui_components::blended_colors;

    pub fn default_border_color(theme: &WarpTheme) -> ColorU {
        blended_colors::neutral_2(theme)
    }
}

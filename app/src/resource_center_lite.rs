use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use settings::Setting as _;
use warpui::elements::Empty;
use warpui::{
    AppContext, Element, Entity, EntityId, ModelHandle, SingletonEntity, TypedActionView, View,
    ViewContext, WindowId,
};

use crate::terminal::general_settings::GeneralSettings;
use crate::util::bindings::trigger_to_keystroke;
use warp_errors::report_if_error;

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "A welcome tip shown to new users.",
    rename_all = "snake_case"
)]
pub enum Tip {
    #[schemars(description = "A non-interactive informational hint.")]
    Hint(TipHint),
    #[schemars(description = "An interactive tip that triggers an action when clicked.")]
    Action(TipAction),
}

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(description = "A non-interactive tip hint.", rename_all = "snake_case")]
pub enum TipHint {
    CreateBlock,
    BlockSelect,
    BlockAction,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(description = "An interactive tip action.", rename_all = "snake_case")]
pub enum TipAction {
    CommandPalette,
    SplitPane,
    ThemePicker,
    HistorySearch,
    CommandSearch,
    AiCommandSearch,
    SaveNewLaunchConfig,
    WarpAI,
    OpenWarpDrive,
    Changelog,
    Workflows,
}

impl TipAction {
    pub fn editable_binding_name(&self) -> &'static str {
        match self {
            TipAction::CommandPalette => "workspace:toggle_command_palette",
            TipAction::SplitPane => "pane_group:add_right",
            TipAction::HistorySearch => "input:search_command_history",
            TipAction::CommandSearch => "workspace:show_command_search",
            TipAction::AiCommandSearch => "input:toggle_natural_language_command_search",
            TipAction::ThemePicker => "workspace:show_theme_chooser",
            TipAction::SaveNewLaunchConfig => "workspace:open_launch_config_save_modal",
            TipAction::WarpAI => "workspace:toggle_ai_assistant",
            TipAction::OpenWarpDrive => "workspace:toggle_left_panel",
            TipAction::Changelog => "/changelog",
            TipAction::Workflows => "input:toggle_workflows",
        }
    }

    pub fn keyboard_shortcut(&self, ctx: &mut AppContext) -> Option<warpui::keymap::Keystroke> {
        ctx.editable_bindings()
            .find(|binding| binding.name == self.editable_binding_name())
            .and_then(|binding| trigger_to_keystroke(binding.trigger))
    }
}

#[derive(Default)]
pub struct TipsCompleted {
    pub features_used: HashSet<Tip>,
    pub skipped_or_completed: bool,
    pub gamified_tips_count: Option<usize>,
}

impl Entity for TipsCompleted {
    type Event = ();
}

pub fn mark_feature_used_and_write_to_user_defaults(
    feature: Tip,
    tips_completed: &mut TipsCompleted,
    ctx: &mut AppContext,
) {
    if tips_completed.mark_feature_used(feature) {
        GeneralSettings::handle(ctx).update(ctx, |general_settings, ctx| {
            report_if_error!(
                general_settings
                    .welcome_tips_features_used
                    .set_value(tips_completed.features_used.clone(), ctx)
            );

            if tips_completed.skipped_or_completed {
                report_if_error!(
                    general_settings
                        .welcome_tips_skipped_or_completed
                        .set_value(true, ctx)
                );
            }
        });
    }
}

pub fn skip_tips_and_write_to_user_defaults(tips_completed: &mut TipsCompleted, ctx: &mut AppContext) {
    tips_completed.skipped_or_completed = true;
    GeneralSettings::handle(ctx).update(ctx, |general_settings, ctx| {
        report_if_error!(
            general_settings
                .welcome_tips_skipped_or_completed
                .set_value(true, ctx)
        );
    });
}

pub fn complete_tips_and_write_to_user_defaults(
    tips_completed: &mut TipsCompleted,
    ctx: &mut AppContext,
) {
    tips_completed.skipped_or_completed = true;
    GeneralSettings::handle(ctx).update(ctx, |general_settings, ctx| {
        report_if_error!(
            general_settings
                .welcome_tips_skipped_or_completed
                .set_value(true, ctx)
        );
    });
}

impl TipsCompleted {
    pub fn new(features_used: HashSet<Tip>, skipped_or_completed: bool) -> Self {
        Self {
            features_used,
            skipped_or_completed,
            gamified_tips_count: None,
        }
    }

    pub fn mark_feature_used(&mut self, feature: Tip) -> bool {
        let is_new_value = self.features_used.insert(feature);

        if let Some(total_tips) = self.gamified_tips_count
            && is_new_value
            && self.features_used.len() == total_tips
        {
            self.skipped_or_completed = true;
        }

        is_new_value
    }

    pub fn serialized_tips(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.features_used)
    }

    pub fn completed_count(&self) -> usize {
        self.features_used.len()
    }

    pub fn set_gamified_tips_count(&mut self, total: usize) {
        self.gamified_tips_count = Some(total)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceCenterPage {
    Main,
    Keybindings,
}

#[derive(Debug, Clone)]
pub enum ResourceCenterEvent {
    Close,
    Escape,
}

#[derive(Debug, Clone)]
pub enum ResourceCenterAction {
    Close,
}

pub struct ResourceCenterView {
    current_page: ResourceCenterPage,
}

impl ResourceCenterView {
    pub fn new(
        _ctx: &mut ViewContext<Self>,
        _tips_completed: ModelHandle<TipsCompleted>,
        _changelog_model_handle: ModelHandle<crate::changelog_model::ChangelogModel>,
    ) -> Self {
        Self {
            current_page: ResourceCenterPage::Main,
        }
    }

    pub fn get_current_page(&self) -> ResourceCenterPage {
        self.current_page
    }

    pub fn set_current_page(&mut self, new_page: ResourceCenterPage, ctx: &mut ViewContext<Self>) {
        self.current_page = new_page;
        ctx.notify();
    }

    pub fn set_action_target(
        &mut self,
        _window_id: WindowId,
        _input_id: Option<EntityId>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(ResourceCenterEvent::Close);
    }
}

impl Entity for ResourceCenterView {
    type Event = ResourceCenterEvent;
}

impl TypedActionView for ResourceCenterView {
    type Action = ResourceCenterAction;
}

impl View for ResourceCenterView {
    fn ui_name() -> &'static str {
        "ResourceCenterView"
    }

    fn render(&self, _app: &warpui::AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

pub mod utils {
    pub use super::TOGGLE_RESOURCE_CENTER_KEYBINDING_NAME;
}

pub const TOGGLE_RESOURCE_CENTER_KEYBINDING_NAME: &str = "workspace:toggle_resource_center";

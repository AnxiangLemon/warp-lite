use std::collections::HashSet;
#[cfg(feature = "agent_runtime")]
use std::path::PathBuf;

use warp_core::context_flag::ContextFlag;
use warp_core::features::FeatureFlag;
use warpui::keymap::BindingId;
use warpui::{AppContext, Entity, ModelContext, ModelHandle};
#[cfg(feature = "agent_runtime")]
use warpui::{SingletonEntity, WindowId};

#[cfg(feature = "agent_runtime")]
use super::{conversations, warp_drive};
#[cfg(feature = "agent_runtime")]
use crate::drive::settings::WarpDriveSettings;
use crate::search::QueryFilter;
use crate::search::action::CommandBindingDataSource;
use crate::search::binding_source::BindingSource;
#[cfg(feature = "agent_runtime")]
use crate::search::command_palette::files;
use crate::search::command_palette::mixer::{CommandPaletteItemAction, ItemSummary};
use crate::search::command_palette::new_session::NewSessionDataSource;
#[cfg(feature = "agent_runtime")]
use crate::search::command_palette::repos::RepoDataSource;
use crate::search::command_palette::{CommandPaletteMixer, launch_config, navigation, tabs};
use crate::search::data_source::QueryResult;
#[cfg(feature = "agent_runtime")]
use crate::search::files::model::FileSearchModel;
#[cfg(feature = "agent_runtime")]
use crate::search::mixer::AddAsyncSourceOptions;
use crate::session_management::SessionSource;
#[cfg(feature = "agent_runtime")]
use crate::settings::AISettings;

/// Store of all of the [`crate::search::DataSource`]s for the command palette.
pub struct DataSourceStore {
    actions_data_source: ModelHandle<CommandBindingDataSource>,
    sessions_data_source: ModelHandle<navigation::DataSource>,
    #[cfg(feature = "agent_runtime")]
    warp_drive_data_source: ModelHandle<warp_drive::DataSource>,
    launch_config_data_source: ModelHandle<launch_config::DataSource>,
    new_session_data_source: Option<ModelHandle<NewSessionDataSource>>,
    #[cfg(feature = "agent_runtime")]
    all_conversation_data_source: ModelHandle<conversations::DataSource>,
    #[cfg(feature = "agent_runtime")]
    repo_data_source: ModelHandle<RepoDataSource>,
    tabs_data_source: Option<ModelHandle<tabs::DataSource>>,
}

impl DataSourceStore {
    pub fn new(
        binding_source: ModelHandle<BindingSource>,
        active_session_handle: ModelHandle<SessionSource>,
        #[cfg(feature = "agent_runtime")] window_id: WindowId,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let actions_data_source =
            ctx.add_model(|ctx| CommandBindingDataSource::new(binding_source.clone(), ctx));

        let sessions_data_source =
            ctx.add_model(|_| navigation::DataSource::new(active_session_handle));

        #[cfg(feature = "agent_runtime")]
        let warp_drive_data_source =
            ctx.add_model(|ctx| warp_drive::DataSource::new(window_id, ctx));

        let launch_config_data_source = ctx.add_model(launch_config::DataSource::new);

        let new_session_data_source = (FeatureFlag::ShellSelector.is_enabled()
            && cfg!(feature = "local_tty"))
        .then(|| ctx.add_model(|ctx| NewSessionDataSource::new(binding_source, ctx)));

        #[cfg(feature = "agent_runtime")]
        let all_conversation_data_source: ModelHandle<conversations::DataSource> =
            ctx.add_model(|_| conversations::DataSource::new());

        #[cfg(feature = "agent_runtime")]
        let repo_data_source = ctx.add_model(|_| RepoDataSource::new());

        Self {
            actions_data_source,
            sessions_data_source,
            #[cfg(feature = "agent_runtime")]
            warp_drive_data_source,
            launch_config_data_source,
            new_session_data_source,
            #[cfg(feature = "agent_runtime")]
            all_conversation_data_source,
            #[cfg(feature = "agent_runtime")]
            repo_data_source,
            tabs_data_source: None,
        }
    }

    /// Resets the [`CommandPaletteMixer`] to the set of data sources that are relevant for the command palette.
    pub fn reset_search_mixer(
        &mut self,
        mixer: ModelHandle<CommandPaletteMixer>,
        #[cfg(feature = "agent_runtime")] is_shared_session_viewer: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        mixer.update(ctx, |mixer, ctx| {
            mixer.reset(ctx);

            if ContextFlag::LaunchConfigurations.is_enabled() {
                mixer.add_sync_source(
                    self.launch_config_data_source.clone(),
                    HashSet::from([QueryFilter::LaunchConfigurations]),
                );
            }

            mixer.add_sync_source(
                self.sessions_data_source.clone(),
                HashSet::from([QueryFilter::Sessions]),
            );

            #[cfg(feature = "agent_runtime")]
            if !cfg!(feature = "warp_lite") && WarpDriveSettings::is_warp_drive_enabled(ctx) {
                let mut warp_drive_filters = HashSet::from([
                    QueryFilter::Notebooks,
                    QueryFilter::Plans,
                    QueryFilter::Drive,
                    QueryFilter::Workflows,
                ]);

                warp_drive_filters.insert(QueryFilter::EnvironmentVariables);

                if AISettings::as_ref(ctx).is_any_ai_enabled(ctx) {
                    warp_drive_filters.insert(QueryFilter::AgentModeWorkflows);
                }
                mixer.add_sync_source(self.warp_drive_data_source.clone(), warp_drive_filters);
            }

            mixer.add_sync_source(
                self.actions_data_source.clone(),
                HashSet::from([QueryFilter::Actions]),
            );

            if let Some(new_session_data_source) = &self.new_session_data_source {
                mixer.add_sync_source(
                    new_session_data_source.clone(),
                    HashSet::from([QueryFilter::Actions]),
                );
            }

            #[cfg(feature = "agent_runtime")]
            if FeatureFlag::CommandPaletteFileSearch.is_enabled() && !is_shared_session_viewer {
                let file_search_model = FileSearchModel::as_ref(ctx);
                let is_in_git_repo = file_search_model.repo_root_location(ctx).is_some();

                let files_data_source = if is_in_git_repo {
                    ctx.add_model(|_| files::data_source::FileDataSource::new())
                } else {
                    ctx.add_model(|ctx| files::data_source::FileDataSource::new_current_folder(ctx))
                };
                mixer.add_async_source(
                    files_data_source,
                    HashSet::from([QueryFilter::Files]),
                    AddAsyncSourceOptions {
                        debounce_interval: None,
                        run_in_zero_state: true,
                        run_when_unfiltered: true,
                    },
                    ctx,
                );
            }

            // Add conversation search if AI is enabled
            #[cfg(feature = "agent_runtime")]
            if !cfg!(feature = "warp_lite") && AISettings::as_ref(ctx).is_any_ai_enabled(ctx) {
                mixer.add_sync_source(
                    self.all_conversation_data_source.clone(),
                    HashSet::from([QueryFilter::Conversations]),
                );
            }

            #[cfg(feature = "agent_runtime")]
            if !cfg!(feature = "warp_lite") {
                mixer.add_sync_source(
                    self.repo_data_source.clone(),
                    HashSet::from([QueryFilter::Repos]),
                );
            }

            ctx.notify();
        });
    }

    /// Resets the [`CommandPaletteMixer`] to the set of data sources relevant for the Ctrl+Tab
    /// palette, which shows tabs sorted by MRU order.
    pub fn reset_ctrl_tab_mixer(
        &mut self,
        mixer: ModelHandle<CommandPaletteMixer>,
        tabs: Vec<crate::session_management::TabNavigationData>,
        ctx: &mut ModelContext<Self>,
    ) {
        if self.tabs_data_source.is_none() {
            self.tabs_data_source = Some(ctx.add_model(|_| tabs::DataSource::new()));
        }

        if let Some(tabs_data_source) = &self.tabs_data_source {
            tabs_data_source.update(ctx, |ds, _| ds.set_tabs(tabs));
            mixer.update(ctx, |mixer, ctx| {
                mixer.reset(ctx);
                mixer.add_sync_source(tabs_data_source.clone(), HashSet::from([QueryFilter::Tabs]));
                ctx.notify();
            });
        }
    }

    /// Restores the [`CommandPaletteMixer`] to the sessions-only source for Ctrl+Tab,
    /// undoing any previous `reset_ctrl_tab_mixer` call.
    pub fn restore_ctrl_tab_session_mixer(
        &self,
        mixer: ModelHandle<CommandPaletteMixer>,
        ctx: &mut ModelContext<Self>,
    ) {
        mixer.update(ctx, |mixer, ctx| {
            mixer.reset(ctx);
            mixer.add_sync_source(
                self.sessions_data_source.clone(),
                HashSet::from([QueryFilter::Sessions]),
            );
            ctx.notify();
        });
    }

    /// Returns a [`QueryResult`] from the data sources identified by the `summary`. `None` if none
    /// of the data sources contained an item with given summary.
    pub fn query_result_from_summary(
        &self,
        summary: &ItemSummary,
        app: &AppContext,
    ) -> Option<QueryResult<CommandPaletteItemAction>> {
        if cfg!(feature = "warp_lite")
            && matches!(
                summary,
                ItemSummary::Workflow { .. }
                    | ItemSummary::EnvVarCollection { .. }
                    | ItemSummary::Notebook { .. }
                    | ItemSummary::CloudObject
                    | ItemSummary::Project { .. }
                    | ItemSummary::Conversation { .. }
                    | ItemSummary::NewConversation
                    | ItemSummary::ForkConversation
            )
        {
            return None;
        }
        match summary {
            #[cfg(not(feature = "agent_runtime"))]
            ItemSummary::Workflow { .. }
            | ItemSummary::EnvVarCollection { .. }
            | ItemSummary::Notebook { .. }
            | ItemSummary::File { .. }
            | ItemSummary::Directory { .. }
            | ItemSummary::Conversation { .. } => None,
            ItemSummary::Action { binding_id } => self
                .actions_data_source
                .as_ref(app)
                .query_result(*binding_id),
            #[cfg(feature = "agent_runtime")]
            ItemSummary::Workflow { id } => self
                .warp_drive_data_source
                .as_ref(app)
                .query_result(id, app),
            #[cfg(feature = "agent_runtime")]
            ItemSummary::EnvVarCollection { id } => self
                .warp_drive_data_source
                .as_ref(app)
                .query_result(id, app),
            #[cfg(feature = "agent_runtime")]
            ItemSummary::Notebook { id } => self
                .warp_drive_data_source
                .as_ref(app)
                .query_result(id, app),
            ItemSummary::Session { pane_view_locator } => self
                .sessions_data_source
                .as_ref(app)
                .query_result(*pane_view_locator, app),
            ItemSummary::LaunchConfiguration => {
                // TODO(CLD-205): Launch configurations are not supported in the recent section of the
                // zero state yet.
                None
            }
            ItemSummary::CloudObject => {
                // We don't yet support all cloud objects in the command palette but
                // we have a `ViewInWarpDrive` action that supports all of them, so
                // this is necessary to make the compiler happy.
                None
            }
            ItemSummary::NewSession { id } => self
                .new_session_data_source
                .as_ref()
                .and_then(|source| source.as_ref(app).query_result(id)),
            #[cfg(feature = "agent_runtime")]
            ItemSummary::File {
                path,
                project_directory,
                line_and_column_arg,
            } => {
                // Create a file search item from the summary
                use fuzzy_match::FuzzyMatchResult;

                use crate::search::command_palette::files::search_item::FileSearchItem;

                let search_item = FileSearchItem {
                    path: PathBuf::from(path),
                    project_directory: project_directory.clone(),
                    match_result: FuzzyMatchResult::no_match(),
                    line_and_column_arg: *line_and_column_arg,
                    is_directory: false,
                };
                Some(QueryResult::from(search_item))
            }
            #[cfg(feature = "agent_runtime")]
            ItemSummary::Directory {
                path,
                project_directory,
            } => {
                // Create a directory search item from the summary
                use fuzzy_match::FuzzyMatchResult;

                use crate::search::command_palette::files::search_item::FileSearchItem;

                let search_item = FileSearchItem {
                    path: PathBuf::from(path),
                    project_directory: project_directory.clone(),
                    match_result: FuzzyMatchResult::no_match(),
                    line_and_column_arg: None,
                    is_directory: true,
                };
                Some(QueryResult::from(search_item))
            }
            ItemSummary::Project { path: _ } => {
                // For project summaries, we would need a project data source to reconstruct the item,
                // but this is typically handled by the welcome palette, not the command palette.
                // For now, return None as projects aren't expected in the regular command palette.
                None
            }
            #[cfg(feature = "agent_runtime")]
            ItemSummary::Conversation { id } => conversations::DataSource::query_result(id, app),

            ItemSummary::NewConversation => {
                // The new conversation item should not show up in the recent command list,
                // as its use is specific to the conversation filter.
                None
            }

            ItemSummary::ForkConversation => {
                // The forked conversation item should not show up in the recent command list,
                // as its use is specific to the conversation filter.
                None
            }

            ItemSummary::NoOp => {
                // No-op action (used for non-interactable separator items that don't do anything on click).
                None
            }

            ItemSummary::Tab { .. } => {
                // Tabs are only shown in the ctrl_tab palette, not in recent commands.
                None
            }
        }
    }

    /// Returns a [`QueryResult`] for a binding with `binding_id`. `None` if no result was found
    /// with the given ID.
    pub fn query_result_for_binding_id(
        &self,
        binding_id: BindingId,
        app: &AppContext,
    ) -> Option<QueryResult<CommandPaletteItemAction>> {
        self.query_result_from_summary(&ItemSummary::Action { binding_id }, app)
    }
}

impl Entity for DataSourceStore {
    type Event = ();
}

#[cfg(all(test, feature = "agent_runtime"))]
#[path = "data_sources_tests.rs"]
mod tests;

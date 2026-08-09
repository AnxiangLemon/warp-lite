use std::any::Any;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use pathfinder_color::ColorU;
use serde::{Deserialize, Serialize};
use warpui::elements::{
    AcceptedByDropTarget, Border, ConstrainedBox, Container, CrossAxisAlignment,
    DispatchEventResult, DragAxis, Draggable, DraggableState, DropTarget, DropTargetData,
    EventHandler, Flex, MainAxisSize, ParentElement, Rect, Shrinkable, Text,
};
use warpui::fonts::FamilyId;
use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext};

const SESSION_FILE: &str = "session.json";
const MAX_OUTPUT_BYTES: usize = 64 * 1024;
const SIDEBAR_WIDTH: f32 = 280.;
const HEADER_HEIGHT: f32 = 52.;
const STATUS_HEIGHT: f32 = 32.;
fn bg() -> ColorU {
    ColorU::new(18, 20, 24, 255)
}
fn panel() -> ColorU {
    ColorU::new(28, 31, 38, 255)
}
fn panel_active() -> ColorU {
    ColorU::new(42, 47, 57, 255)
}
fn border_color() -> ColorU {
    ColorU::new(65, 72, 86, 255)
}
fn text_color() -> ColorU {
    ColorU::new(225, 230, 238, 255)
}
fn muted() -> ColorU {
    ColorU::new(150, 160, 176, 255)
}
fn accent() -> ColorU {
    ColorU::new(91, 164, 255, 255)
}
fn terminal_text() -> ColorU {
    ColorU::new(214, 220, 228, 255)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LiteShell {
    PowerShell,
    Cmd,
    GitBash,
    Wsl,
}

impl LiteShell {
    fn label(&self) -> &'static str {
        match self {
            Self::PowerShell => "PowerShell",
            Self::Cmd => "CMD",
            Self::GitBash => "Git Bash",
            Self::Wsl => "WSL",
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::PowerShell => Self::Cmd,
            Self::Cmd => Self::GitBash,
            Self::GitBash => Self::Wsl,
            Self::Wsl => Self::PowerShell,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LitePane {
    pub cwd: PathBuf,
    pub shell: LiteShell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiteTab {
    pub title: String,
    pub panes: Vec<LitePane>,
    pub split: Option<SplitDirection>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiteGroup {
    pub title: String,
    pub collapsed: bool,
    pub tabs: Vec<LiteTab>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiteSession {
    pub groups: Vec<LiteGroup>,
    pub active_group: usize,
    pub active_tab: usize,
    pub font_size: f32,
    pub dark_theme: bool,
}

impl Default for LiteSession {
    fn default() -> Self {
        Self {
            groups: vec![LiteGroup {
                title: "Main".to_owned(),
                collapsed: false,
                tabs: vec![LiteTab::new("Terminal 1")],
            }],
            active_group: 0,
            active_tab: 0,
            font_size: 14.,
            dark_theme: true,
        }
    }
}

impl LiteTab {
    fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            panes: vec![LitePane {
                cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                shell: LiteShell::PowerShell,
            }],
            split: None,
        }
    }
}

#[derive(Debug)]
struct LitePaneRuntime {
    _child: Arc<Mutex<Child>>,
    stdin: Arc<Mutex<ChildStdin>>,
    output: Arc<Mutex<String>>,
}

#[derive(Clone, Debug)]
pub enum LiteAction {
    NewTab,
    CloseTab,
    RenameTab,
    NewGroup,
    RenameGroup,
    ToggleGroup(usize),
    MoveTabToGroup,
    MoveTabLeft,
    MoveTabRight,
    DropTab {
        from_group: usize,
        from_tab: usize,
        to_group: usize,
        to_tab: usize,
    },
    Split(SplitDirection),
    CycleShell,
    ToggleSettings,
    FontSizeUp,
    FontSizeDown,
    ToggleTheme,
    SelectGroup(usize),
    SelectTab(usize, usize),
    SendKey(String),
    RenameInput(String),
    RenameBackspace,
    FinishRename,
    CancelRename,
}

#[derive(Clone, Copy, Debug)]
enum RenameTarget {
    Tab,
    Group,
}

#[derive(Debug)]
struct LiteTabDropTargetData {
    group: usize,
    tab: usize,
}

impl DropTargetData for LiteTabDropTargetData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Entity for LiteRootView {
    type Event = ();
}

pub struct LiteRootView {
    session: LiteSession,
    font_family: FamilyId,
    show_settings: bool,
    rename_buffer: Option<String>,
    rename_target: Option<RenameTarget>,
    runtimes: Vec<Vec<Vec<LitePaneRuntime>>>,
    drag_states: Vec<Vec<DraggableState>>,
}

impl LiteRootView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let session = load_session();
        let font_family = warpui::fonts::Cache::handle(ctx).update(ctx, |cache, _| {
            cache
                .load_system_font("Cascadia Mono")
                .or_else(|_| cache.load_system_font("Consolas"))
                .expect("a monospace system font is required")
        });
        let mut view = Self {
            runtimes: Vec::new(),
            drag_states: Vec::new(),
            session,
            font_family,
            show_settings: false,
            rename_buffer: None,
            rename_target: None,
        };
        view.start_missing_panes();
        view.schedule_refresh(ctx);
        ctx.focus_self();
        view
    }

    fn schedule_refresh(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.spawn(
            warpui::r#async::Timer::after(Duration::from_millis(50)),
            |view, _, ctx| {
                ctx.notify();
                view.schedule_refresh(ctx);
            },
        );
    }

    fn start_missing_panes(&mut self) {
        self.sync_drag_states();
        self.runtimes.clear();
        for group in &self.session.groups {
            let mut group_runtimes = Vec::new();
            for tab in &group.tabs {
                group_runtimes.push(tab.panes.iter().map(LitePaneRuntime::start).collect());
            }
            self.runtimes.push(group_runtimes);
        }
    }

    fn sync_drag_states(&mut self) {
        self.drag_states
            .resize_with(self.session.groups.len(), Vec::new);
        for (group_index, group) in self.session.groups.iter().enumerate() {
            self.drag_states[group_index].resize_with(group.tabs.len(), DraggableState::default);
            self.drag_states[group_index].truncate(group.tabs.len());
        }
        self.drag_states.truncate(self.session.groups.len());
    }

    fn persist(&self) {
        let path = session_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_vec_pretty(&self.session) {
            let temp = path.with_extension("json.tmp");
            if fs::write(&temp, data).is_ok() {
                let _ = fs::rename(temp, path);
            }
        }
    }

    fn active_tab_mut(&mut self) -> Option<&mut LiteTab> {
        self.session
            .groups
            .get_mut(self.session.active_group)
            .and_then(|group| group.tabs.get_mut(self.session.active_tab))
    }

    fn active_runtime(&self) -> Option<&LitePaneRuntime> {
        self.runtimes
            .get(self.session.active_group)
            .and_then(|group| group.get(self.session.active_tab))
            .and_then(|panes| panes.first())
    }

    fn handle_action(&mut self, action: &LiteAction) {
        match action {
            LiteAction::NewTab => {
                let group = self.session.active_group;
                let title = format!("Terminal {}", tab_count(&self.session) + 1);
                self.session.groups[group].tabs.push(LiteTab::new(title));
                self.session.active_tab = self.session.groups[group].tabs.len() - 1;
                self.start_missing_panes();
            }
            LiteAction::CloseTab => {
                let group = self.session.active_group;
                if self.session.groups[group].tabs.len() > 1 {
                    self.session.groups[group]
                        .tabs
                        .remove(self.session.active_tab);
                    self.session.active_tab = self.session.active_tab.saturating_sub(1);
                    self.start_missing_panes();
                }
            }
            LiteAction::RenameTab => {
                self.rename_target = Some(RenameTarget::Tab);
                self.rename_buffer = Some(
                    self.active_tab_mut()
                        .map(|tab| tab.title.clone())
                        .unwrap_or_default(),
                );
            }
            LiteAction::NewGroup => {
                self.session.groups.push(LiteGroup {
                    title: format!("Group {}", self.session.groups.len() + 1),
                    collapsed: false,
                    tabs: vec![LiteTab::new("Terminal 1")],
                });
                self.session.active_group = self.session.groups.len() - 1;
                self.session.active_tab = 0;
                self.start_missing_panes();
            }
            LiteAction::RenameGroup => {
                self.rename_target = Some(RenameTarget::Group);
                self.rename_buffer = self
                    .session
                    .groups
                    .get(self.session.active_group)
                    .map(|group| group.title.clone());
            }
            LiteAction::ToggleGroup(group_index) => {
                if let Some(group) = self.session.groups.get_mut(*group_index) {
                    group.collapsed = !group.collapsed;
                }
            }
            LiteAction::MoveTabToGroup => {
                if self.session.groups.len() > 1 {
                    let source = self.session.active_group;
                    let target = (source + 1) % self.session.groups.len();
                    if self.session.groups[source].tabs.len() > 1 {
                        let tab = self.session.groups[source]
                            .tabs
                            .remove(self.session.active_tab);
                        self.session.groups[target].tabs.push(tab);
                        self.session.active_group = target;
                        self.session.active_tab = self.session.groups[target].tabs.len() - 1;
                        self.start_missing_panes();
                    }
                }
            }
            LiteAction::MoveTabLeft => self.move_tab(-1),
            LiteAction::MoveTabRight => self.move_tab(1),
            LiteAction::DropTab {
                from_group,
                from_tab,
                to_group,
                to_tab,
            } => self.drop_tab(*from_group, *from_tab, *to_group, *to_tab),
            LiteAction::Split(direction) => {
                if let Some(tab) = self.active_tab_mut()
                    && tab.split.is_none()
                {
                    let pane = tab.panes.first().cloned();
                    tab.split = Some(direction.clone());
                    if let Some(mut pane) = pane {
                        pane.cwd = pane.cwd.clone();
                        tab.panes.push(pane);
                    }
                    self.start_missing_panes();
                }
            }
            LiteAction::CycleShell => {
                if let Some(tab) = self.active_tab_mut()
                    && let Some(pane) = tab.panes.first_mut()
                {
                    pane.shell = pane.shell.next();
                    self.start_missing_panes();
                }
            }
            LiteAction::ToggleSettings => self.show_settings = !self.show_settings,
            LiteAction::FontSizeUp => {
                self.session.font_size = (self.session.font_size + 1.).min(24.)
            }
            LiteAction::FontSizeDown => {
                self.session.font_size = (self.session.font_size - 1.).max(10.)
            }
            LiteAction::ToggleTheme => self.session.dark_theme = !self.session.dark_theme,
            LiteAction::SelectGroup(group) => {
                if let Some(group_state) = self.session.groups.get(*group) {
                    self.session.active_group = *group;
                    self.session.active_tab = self
                        .session
                        .active_tab
                        .min(group_state.tabs.len().saturating_sub(1));
                }
            }
            LiteAction::SelectTab(group, tab) => {
                if self
                    .session
                    .groups
                    .get(*group)
                    .is_some_and(|g| g.tabs.get(*tab).is_some())
                {
                    self.session.active_group = *group;
                    self.session.active_tab = *tab;
                    self.session.groups[*group].collapsed = false;
                }
            }
            LiteAction::SendKey(key) => self.send_key(key),
            LiteAction::RenameInput(value) => {
                if let Some(buffer) = self.rename_buffer.as_mut() {
                    buffer.push_str(value);
                }
            }
            LiteAction::RenameBackspace => {
                if let Some(buffer) = self.rename_buffer.as_mut() {
                    buffer.pop();
                }
            }
            LiteAction::FinishRename => {
                let Some(name) = self.rename_buffer.take() else {
                    return;
                };
                let target = self.rename_target.take();
                let name = name.trim().to_owned();
                if !name.is_empty() {
                    match target {
                        Some(RenameTarget::Tab) => {
                            if let Some(tab) = self.active_tab_mut() {
                                tab.title = name;
                            }
                        }
                        Some(RenameTarget::Group) => {
                            if let Some(group) =
                                self.session.groups.get_mut(self.session.active_group)
                            {
                                group.title = name;
                            }
                        }
                        None => {}
                    }
                }
            }
            LiteAction::CancelRename => {
                self.rename_buffer = None;
                self.rename_target = None;
            }
        }
        self.persist();
    }

    fn move_tab(&mut self, offset: isize) {
        let group = &mut self.session.groups[self.session.active_group];
        let index = self.session.active_tab as isize;
        let target = index + offset;
        if target >= 0 && target < group.tabs.len() as isize {
            group.tabs.swap(index as usize, target as usize);
            self.session.active_tab = target as usize;
            self.start_missing_panes();
        }
    }

    fn drop_tab(&mut self, from_group: usize, from_tab: usize, to_group: usize, to_tab: usize) {
        if from_group >= self.session.groups.len()
            || to_group >= self.session.groups.len()
            || from_tab >= self.session.groups[from_group].tabs.len()
            || to_tab >= self.session.groups[to_group].tabs.len()
            || (from_group == to_group && from_tab == to_tab)
        {
            return;
        }

        let tab = self.session.groups[from_group].tabs.remove(from_tab);
        let insert_at = if from_group == to_group && from_tab < to_tab {
            to_tab.saturating_sub(1)
        } else {
            to_tab
        };
        self.session.groups[to_group].tabs.insert(insert_at, tab);
        self.session.active_group = to_group;
        self.session.active_tab = insert_at;
        self.start_missing_panes();
    }

    fn send_key(&mut self, key: &str) {
        let Some(runtime) = self.active_runtime() else {
            return;
        };
        let bytes = terminal_bytes(key);
        if let Ok(mut stdin) = runtime.stdin.lock() {
            let _ = stdin.write_all(&bytes);
            let _ = stdin.flush();
        }
    }

    fn text(&self, value: impl Into<String>, size: f32, color: ColorU) -> Box<dyn Element> {
        Text::new(value.into(), self.font_family, size)
            .with_color(color)
            .finish()
    }

    fn spacer(&self) -> Box<dyn Element> {
        self.text("", 1., self.theme_bg())
    }

    fn theme_bg(&self) -> ColorU {
        if self.session.dark_theme {
            bg()
        } else {
            ColorU::new(241, 244, 248, 255)
        }
    }

    fn theme_panel(&self) -> ColorU {
        if self.session.dark_theme {
            panel()
        } else {
            ColorU::new(255, 255, 255, 255)
        }
    }

    fn theme_border(&self) -> ColorU {
        if self.session.dark_theme {
            border_color()
        } else {
            ColorU::new(195, 202, 214, 255)
        }
    }

    fn theme_text(&self) -> ColorU {
        if self.session.dark_theme {
            text_color()
        } else {
            ColorU::new(30, 36, 46, 255)
        }
    }

    fn theme_muted(&self) -> ColorU {
        if self.session.dark_theme {
            muted()
        } else {
            ColorU::new(90, 100, 116, 255)
        }
    }

    fn button(&self, label: impl Into<String>, action: LiteAction) -> Box<dyn Element> {
        let child = Container::new(
            ConstrainedBox::new(self.text(label, self.session.font_size, self.theme_text()))
                .with_height(24.)
                .finish(),
        )
        .with_uniform_padding(6.)
        .with_background_color(self.theme_panel())
        .with_border(Border::all(1.).with_border_color(self.theme_border()))
        .finish();
        EventHandler::new(child)
            .on_left_mouse_down(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish()
    }

    fn clickable(
        &self,
        label: impl Into<String>,
        action: LiteAction,
        color: ColorU,
        background: ColorU,
        height: f32,
    ) -> Box<dyn Element> {
        let child = Container::new(
            ConstrainedBox::new(self.text(label, self.session.font_size - 1., color))
                .with_height(height - 12.)
                .finish(),
        )
        .with_uniform_padding(6.)
        .with_background_color(background)
        .with_border(Border::all(1.).with_border_color(self.theme_border()))
        .finish();
        EventHandler::new(child)
            .on_left_mouse_down(move |ctx, _, _| {
                ctx.dispatch_typed_action(action.clone());
                DispatchEventResult::StopPropagation
            })
            .finish()
    }

    fn sidebar_tab(&self, group: usize, tab: usize, item: &LiteTab) -> Box<dyn Element> {
        let active = self.session.active_group == group && self.session.active_tab == tab;
        let label = if active {
            format!(">  {}", item.title)
        } else {
            format!("   {}", item.title)
        };
        let child = self.clickable(
            label,
            LiteAction::SelectTab(group, tab),
            if active { accent() } else { self.theme_text() },
            if active {
                if self.session.dark_theme {
                    panel_active()
                } else {
                    ColorU::new(226, 235, 248, 255)
                }
            } else {
                self.theme_panel()
            },
            42.,
        );
        let drag_state = self.drag_states[group][tab].clone();
        let draggable = Draggable::new(drag_state, child)
            .with_drag_axis(DragAxis::VerticalOnly)
            .with_accepted_by_drop_target_fn(|data, _| {
                if data.as_any().is::<LiteTabDropTargetData>() {
                    AcceptedByDropTarget::Yes
                } else {
                    AcceptedByDropTarget::No
                }
            })
            .on_drop(move |ctx, _, _, data| {
                if let Some(target) =
                    data.and_then(|data| data.as_any().downcast_ref::<LiteTabDropTargetData>())
                {
                    ctx.dispatch_typed_action(LiteAction::DropTab {
                        from_group: group,
                        from_tab: tab,
                        to_group: target.group,
                        to_tab: target.tab,
                    });
                }
            })
            .finish();
        let drop_target = DropTarget::new(draggable, LiteTabDropTargetData { group, tab }).finish();
        ConstrainedBox::new(drop_target)
            .with_width(SIDEBAR_WIDTH - 24.)
            .finish()
    }

    fn render_group(&self, group_index: usize, group: &LiteGroup) -> Box<dyn Element> {
        let collapse = if group.collapsed { ">" } else { "v" };
        let header = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(6.)
            .with_child(self.clickable(
                collapse,
                LiteAction::ToggleGroup(group_index),
                self.theme_muted(),
                self.theme_panel(),
                30.,
            ))
            .with_child(self.clickable(
                format!("{}  ({} tabs)", group.title, group.tabs.len()),
                LiteAction::SelectGroup(group_index),
                self.theme_text(),
                self.theme_panel(),
                30.,
            ))
            .with_child(self.clickable(
                "...",
                LiteAction::RenameGroup,
                self.theme_muted(),
                self.theme_panel(),
                30.,
            ));
        let mut group_view = Flex::column()
            .with_spacing(4.)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(header.finish());
        if !group.collapsed {
            for (tab_index, tab) in group.tabs.iter().enumerate() {
                group_view = group_view.with_child(self.sidebar_tab(group_index, tab_index, tab));
            }
        }
        group_view.finish()
    }

    fn render_sidebar(&self) -> Box<dyn Element> {
        let search = Container::new(
            ConstrainedBox::new(self.text("Search tabs...", 13., self.theme_muted()))
                .with_height(28.)
                .finish(),
        )
        .with_uniform_padding(8.)
        .with_background_color(self.theme_bg())
        .with_border(Border::all(1.).with_border_color(self.theme_border()))
        .finish();

        let mut groups = Flex::column()
            .with_spacing(12.)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
        for (group_index, group) in self.session.groups.iter().enumerate() {
            groups = groups.with_child(self.render_group(group_index, group));
        }

        let sidebar = Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_spacing(10.)
            .with_child(search)
            .with_child(self.clickable(
                "+  New group",
                LiteAction::NewGroup,
                self.theme_text(),
                self.theme_panel(),
                36.,
            ))
            .with_child(Shrinkable::new(1., groups.finish()).finish())
            .with_child(self.clickable(
                "Settings",
                LiteAction::ToggleSettings,
                self.theme_text(),
                self.theme_panel(),
                38.,
            ));

        ConstrainedBox::new(
            Container::new(sidebar.finish())
                .with_uniform_padding(12.)
                .with_background_color(self.theme_panel())
                .with_border(Border::all(1.).with_border_color(self.theme_border()))
                .finish(),
        )
        .with_width(SIDEBAR_WIDTH)
        .finish()
    }

    fn render_terminal(&self) -> Box<dyn Element> {
        let Some(tab) = self
            .session
            .groups
            .get(self.session.active_group)
            .and_then(|group| group.tabs.get(self.session.active_tab))
        else {
            return Rect::new().with_background_color(self.theme_bg()).finish();
        };
        let direction = tab.split.clone().unwrap_or(SplitDirection::Vertical);
        let mut panes = match direction {
            SplitDirection::Horizontal => Flex::row().with_spacing(1.),
            SplitDirection::Vertical => Flex::column().with_spacing(1.),
        };
        panes = panes
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch);
        for (index, pane) in tab.panes.iter().enumerate() {
            let output = self
                .runtimes
                .get(self.session.active_group)
                .and_then(|group| group.get(self.session.active_tab))
                .and_then(|panes| panes.get(index))
                .and_then(|runtime| runtime.output.lock().ok().map(|output| output.clone()))
                .unwrap_or_else(|| {
                    "Unable to start shell. Check the selected shell in Settings.\n".to_owned()
                });
            let output = tail(&output, MAX_OUTPUT_BYTES);
            let pane_label = format!("{}  {}", pane.shell.label(), pane.cwd.display());
            let output_view = Container::new(
                Flex::column()
                    .with_main_axis_size(MainAxisSize::Max)
                    .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .with_child(self.text(output, self.session.font_size, terminal_text()))
                    .finish(),
            )
            .with_uniform_padding(10.)
            .with_background_color(ColorU::new(10, 12, 15, 255))
            .finish();
            let pane_view = Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                .with_child(
                    Container::new(self.text(pane_label, 12., self.theme_muted()))
                        .with_uniform_padding(6.)
                        .finish(),
                )
                .with_child(Shrinkable::new(1., output_view).finish())
                .finish();
            panes = panes.with_child(
                Shrinkable::new(
                    1.,
                    Container::new(pane_view)
                        .with_border(Border::all(1.).with_border_color(self.theme_border()))
                        .finish(),
                )
                .finish(),
            );
        }
        Container::new(panes.finish())
            .with_background_color(self.theme_bg())
            .finish()
    }

    fn render_settings(&self) -> Box<dyn Element> {
        Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(6.)
            .with_child(self.text("Local Settings", self.session.font_size, self.theme_text()))
            .with_child(Shrinkable::new(1., self.spacer()).finish())
            .with_child(self.button("Font -", LiteAction::FontSizeDown))
            .with_child(self.button("Font +", LiteAction::FontSizeUp))
            .with_child(self.button("Theme", LiteAction::ToggleTheme))
            .with_child(self.button("Shell", LiteAction::CycleShell))
            .finish()
    }

    fn render_active_tab_header(&self) -> Box<dyn Element> {
        let Some(tab) = self.active_tab_mut_for_render() else {
            return Rect::new()
                .with_background_color(self.theme_panel())
                .finish();
        };
        let shell = tab
            .panes
            .first()
            .map_or("No shell", |pane| pane.shell.label());
        let heading = format!("{}  /  {}", tab.title, shell);
        let row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(6.)
            .with_child(self.text(heading, self.session.font_size, self.theme_text()))
            .with_child(Shrinkable::new(1., self.spacer()).finish())
            .with_child(self.button("Rename", LiteAction::RenameTab))
            .with_child(self.button("Move", LiteAction::MoveTabToGroup))
            .with_child(self.button("<", LiteAction::MoveTabLeft))
            .with_child(self.button(">", LiteAction::MoveTabRight))
            .with_child(self.button("Split LR", LiteAction::Split(SplitDirection::Horizontal)))
            .with_child(self.button("Split TB", LiteAction::Split(SplitDirection::Vertical)))
            .with_child(self.button("Close", LiteAction::CloseTab));
        Container::new(row.finish())
            .with_uniform_padding(8.)
            .with_background_color(self.theme_panel())
            .with_border(Border::all(1.).with_border_color(self.theme_border()))
            .finish()
    }

    fn render_main_content(&self) -> Box<dyn Element> {
        let mut main = Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_spacing(8.)
            .with_child(
                ConstrainedBox::new(self.render_active_tab_header())
                    .with_height(52.)
                    .finish(),
            );
        if self.show_settings {
            main = main.with_child(
                ConstrainedBox::new(
                    Container::new(self.render_settings())
                        .with_uniform_padding(8.)
                        .with_background_color(self.theme_panel())
                        .with_border(Border::all(1.).with_border_color(self.theme_border()))
                        .finish(),
                )
                .with_height(52.)
                .finish(),
            );
        }
        main = main
            .with_child(Shrinkable::new(1., self.render_terminal()).finish())
            .with_child(
                ConstrainedBox::new(
                    Container::new(self.text(self.status_text(), 12., self.theme_muted()))
                        .with_uniform_padding(8.)
                        .with_background_color(self.theme_panel())
                        .with_border(Border::all(1.).with_border_color(self.theme_border()))
                        .finish(),
                )
                .with_height(STATUS_HEIGHT)
                .finish(),
            );
        Container::new(main.finish())
            .with_uniform_padding(10.)
            .with_background_color(self.theme_bg())
            .finish()
    }

    fn status_text(&self) -> String {
        let current_shell = self
            .active_tab_mut_for_render()
            .and_then(|tab| tab.panes.first())
            .map_or("No shell", |pane| pane.shell.label());
        self.rename_buffer.as_ref().map_or_else(
            || format!("{}  |  Local session", current_shell),
            |buffer| format!("Rename: {buffer}  |  Enter confirm  Escape cancel"),
        )
    }
}

impl View for LiteRootView {
    fn ui_name() -> &'static str {
        "WarpLiteRootView"
    }

    fn render(&self, _: &AppContext) -> Box<dyn Element> {
        let header = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(12.)
            .with_child(self.text("Warp Lite", 18., self.theme_text()))
            .with_child(self.text("LOCAL ONLY", 11., accent()))
            .with_child(Shrinkable::new(1., self.spacer()).finish())
            .with_child(self.button("+ Tab", LiteAction::NewTab))
            .with_child(self.button("Settings", LiteAction::ToggleSettings));
        let header = ConstrainedBox::new(
            Container::new(header.finish())
                .with_uniform_padding(6.)
                .with_background_color(self.theme_panel())
                .with_border(Border::all(1.).with_border_color(self.theme_border()))
                .finish(),
        )
        .with_height(HEADER_HEIGHT)
        .finish();

        let workspace = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(self.render_sidebar())
            .with_child(Shrinkable::new(1., self.render_main_content()).finish())
            .finish();
        let content = Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(header)
            .with_child(Shrinkable::new(1., workspace).finish())
            .finish();

        let renaming = self.rename_buffer.is_some();
        let action_handler = EventHandler::new(content)
            .with_always_handle()
            .on_keydown(move |ctx, _, key| {
                let action = if renaming && key.key == "Enter" {
                    LiteAction::FinishRename
                } else if renaming && key.key == "Escape" {
                    LiteAction::CancelRename
                } else if renaming && key.key == "Backspace" {
                    LiteAction::RenameBackspace
                } else if renaming && !key.ctrl && !key.alt && !key.cmd {
                    LiteAction::RenameInput(key.key.clone())
                } else if key.ctrl && key.shift && key.key.eq_ignore_ascii_case("t") {
                    LiteAction::NewTab
                } else if key.ctrl && key.key.eq_ignore_ascii_case("w") {
                    LiteAction::CloseTab
                } else if key.ctrl && key.shift && key.key.eq_ignore_ascii_case("h") {
                    LiteAction::Split(SplitDirection::Horizontal)
                } else if key.ctrl && key.shift && key.key.eq_ignore_ascii_case("v") {
                    LiteAction::Split(SplitDirection::Vertical)
                } else if key.ctrl && key.key == "," {
                    LiteAction::ToggleSettings
                } else {
                    LiteAction::SendKey(key.key.clone())
                };
                ctx.dispatch_typed_action(action);
                DispatchEventResult::StopPropagation
            })
            .finish();
        Container::new(action_handler)
            .with_uniform_padding(8.)
            .with_background_color(self.theme_bg())
            .finish()
    }
}

impl TypedActionView for LiteRootView {
    type Action = LiteAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        self.handle_action(action);
        ctx.notify();
    }
}

impl LitePaneRuntime {
    fn start(pane: &LitePane) -> Self {
        let mut command = shell_command(&pane.shell);
        command
            .current_dir(&pane.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }
        let child = command.spawn();
        let (child, stdin, output) = match child {
            Ok(mut child) => {
                let stdin = child.stdin.take();
                let output = Arc::new(Mutex::new(String::new()));
                if let Some(stdout) = child.stdout.take() {
                    spawn_reader(stdout, output.clone());
                }
                if let Some(stderr) = child.stderr.take() {
                    spawn_reader(stderr, output.clone());
                }
                (child, stdin, output)
            }
            Err(error) => {
                let output = Arc::new(Mutex::new(format!(
                    "Failed to start {}: {error}\n",
                    pane.shell.label()
                )));
                let mut fallback = Command::new(shell_command_name());
                fallback
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());
                let mut child = fallback.spawn().expect("fallback shell");
                let stdin = child.stdin.take();
                if let Some(stdout) = child.stdout.take() {
                    spawn_reader(stdout, output.clone());
                }
                if let Some(stderr) = child.stderr.take() {
                    spawn_reader(stderr, output.clone());
                }
                (child, stdin, output)
            }
        };
        Self {
            _child: Arc::new(Mutex::new(child)),
            stdin: Arc::new(Mutex::new(stdin.expect("shell stdin unavailable"))),
            output,
        }
    }
}

fn shell_command(shell: &LiteShell) -> Command {
    match shell {
        LiteShell::PowerShell => {
            let mut command = Command::new(if cfg!(windows) {
                "powershell.exe"
            } else {
                "pwsh"
            });
            command.args(["-NoLogo", "-NoProfile"]);
            command
        }
        LiteShell::Cmd => {
            let mut command = Command::new(if cfg!(windows) { "cmd.exe" } else { "sh" });
            command.arg(if cfg!(windows) { "/K" } else { "-i" });
            command
        }
        LiteShell::GitBash => {
            let path = [
                PathBuf::from(r"C:\Program Files\Git\bin\bash.exe"),
                PathBuf::from(r"C:\Program Files (x86)\Git\bin\bash.exe"),
            ]
            .into_iter()
            .find(|path| path.is_file())
            .unwrap_or_else(|| PathBuf::from("bash"));
            let mut command = Command::new(path);
            command.args(["--noprofile", "--norc", "-i"]);
            command
        }
        LiteShell::Wsl => {
            let mut command = Command::new(if cfg!(windows) { "wsl.exe" } else { "bash" });
            command.arg("-e").arg("bash");
            command
        }
    }
}

fn shell_command_name() -> &'static str {
    if cfg!(windows) { "cmd.exe" } else { "sh" }
}

fn spawn_reader<R: Read + Send + 'static>(mut reader: R, output: Arc<Mutex<String>>) {
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(size) => {
                    if let Ok(mut output) = output.lock() {
                        output.push_str(&String::from_utf8_lossy(&buffer[..size]));
                        let excess = output.len().saturating_sub(MAX_OUTPUT_BYTES);
                        if excess > 0 {
                            output.drain(..excess);
                        }
                    }
                }
            }
        }
    });
}

fn terminal_bytes(key: &str) -> Vec<u8> {
    match key {
        "Enter" | "Return" => vec![b'\r'],
        "Backspace" => vec![8],
        "Tab" => vec![b'\t'],
        "Escape" => vec![27],
        "ArrowUp" => b"\x1b[A".to_vec(),
        "ArrowDown" => b"\x1b[B".to_vec(),
        "ArrowRight" => b"\x1b[C".to_vec(),
        "ArrowLeft" => b"\x1b[D".to_vec(),
        _ => key.as_bytes().to_vec(),
    }
}

fn tail(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }
    value[value.len() - max_bytes..].to_owned()
}

fn tab_count(session: &LiteSession) -> usize {
    session.groups.iter().map(|group| group.tabs.len()).sum()
}

fn session_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join("WarpLite")
        .join(SESSION_FILE)
}

fn load_session() -> LiteSession {
    fs::read(session_path())
        .ok()
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default()
}

impl LiteRootView {
    fn active_tab_mut_for_render(&self) -> Option<&LiteTab> {
        self.session
            .groups
            .get(self.session.active_group)
            .and_then(|group| group.tabs.get(self.session.active_tab))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_round_trips_window_tree() {
        let session = LiteSession::default();
        let encoded = serde_json::to_vec(&session).unwrap();
        let restored: LiteSession = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored.groups.len(), 1);
        assert_eq!(restored.groups[0].tabs[0].panes.len(), 1);
    }

    #[test]
    fn shell_cycle_contains_only_lite_shells() {
        let shell = LiteShell::PowerShell.next().next().next().next();
        assert_eq!(shell, LiteShell::PowerShell);
    }
}

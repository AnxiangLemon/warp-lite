pub mod notifications {
    pub mod item_rendering {}

    pub struct NotificationSourceAgent;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum NotificationFilter {
        #[default]
        All,
    }

    pub mod toast_stack {
        use warpui::elements::{Element, Empty};
        use warpui::{AppContext, Entity, View, ViewContext};

        pub struct AgentNotificationToastStack;

        impl Entity for AgentNotificationToastStack {
            type Event = ();
        }

        impl View for AgentNotificationToastStack {
            fn ui_name() -> &'static str {
                "AgentNotificationToastStackLite"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn Element> {
                Empty::new().finish()
            }
        }
    }

    pub mod view {
        use warpui::elements::{Element, Empty};
        use warpui::{AppContext, Entity, View, ViewContext};

        #[derive(Clone, Debug)]
        pub enum NotificationMailboxViewEvent {
            NavigateToTerminal { terminal_view_id: warpui::EntityId },
            Dismissed,
        }

        pub struct NotificationMailboxView;

        impl NotificationMailboxView {
            pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
                Self
            }
        }

        impl Entity for NotificationMailboxView {
            type Event = NotificationMailboxViewEvent;
        }

        impl View for NotificationMailboxView {
            fn ui_name() -> &'static str {
                "NotificationMailboxViewLite"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn Element> {
                Empty::new().finish()
            }
        }
    }
}

pub mod telemetry {
    pub struct AgentManagementTelemetryEvent;
    pub use crate::ai::artifacts::ArtifactType;
}

pub mod view {
    use warpui::elements::{Element, Empty};
    use warpui::{AppContext, Entity, View, ViewContext};

    #[derive(Clone, Debug)]
    pub enum AgentManagementViewEvent {
        OpenNewTabAndRunWorkflow(()),
        OpenPlanNotebook { notebook_uid: String },
    }

    pub struct AgentManagementView;

    impl AgentManagementView {
        pub fn new(_filters: (), _ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for AgentManagementView {
        type Event = AgentManagementViewEvent;
    }

    impl View for AgentManagementView {
        fn ui_name() -> &'static str {
            "AgentManagementViewLite"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

pub struct AgentManagementEvent;

pub fn init(_app: &mut warpui::AppContext) {}

use warpui::{Entity, ModelContext, SingletonEntity};

pub struct AgentNotificationsModel;

impl AgentNotificationsModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for AgentNotificationsModel {
    type Event = ();
}

impl SingletonEntity for AgentNotificationsModel {}

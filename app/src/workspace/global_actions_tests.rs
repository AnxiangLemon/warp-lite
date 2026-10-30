use std::sync::mpsc::{TryRecvError, sync_channel};

use warpui::{App, SingletonEntity};

use super::{CrossWindowTabDrag, save_app};
use crate::GlobalResourceHandlesProvider;
use crate::test_util::settings::initialize_history_persistence_for_tests;

#[test]
fn saving_without_windows_preserves_lite_session() {
    App::test((), |mut app| async move {
        initialize_history_persistence_for_tests(&mut app);
        app.add_singleton_model(|_| CrossWindowTabDrag::new());
        let (sender, receiver) = sync_channel(1);
        GlobalResourceHandlesProvider::handle(&app).update(&mut app, |provider, _| {
            provider.set_model_event_sender_for_test(sender);
        });

        app.update(|ctx| save_app(&(), ctx));

        assert!(matches!(receiver.try_recv(), Err(TryRecvError::Empty)));
    });
}

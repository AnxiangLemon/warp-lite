use futures_lite::future::block_on;
use warpui::App;

use super::{TelemetryApi, TelemetryCollector, TelemetryEvent};
use crate::server::CloudServicesUnavailable;
use crate::settings::PrivacySettingsSnapshot;

#[test]
fn disabled_events_do_not_build_payloads_through_the_shared_trait() {
    use warp_core::telemetry::TelemetryEvent as TelemetryEventTrait;

    let event = TelemetryEvent::DatabaseWriteError("local error details".to_owned());
    assert_eq!(event.payload(), None);
    assert_eq!(TelemetryEventTrait::payload(&event), None);
}

#[test]
fn disabled_transport_rejects_requests_without_reading_or_rewriting_event_files() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("pending-telemetry.json");
    let original = b"not JSON: must remain untouched";
    std::fs::write(&path, original).unwrap();
    let api = TelemetryApi::new();
    let settings = PrivacySettingsSnapshot::mock();
    let errors = [
        block_on(api.flush_events(settings)).map(|_| ()),
        block_on(api.flush_persisted_events_to_rudder(&path, settings)),
        api.flush_and_persist_events(20, settings),
        block_on(api.send_telemetry_event(
            None,
            "offline-test".to_string(),
            TelemetryEvent::DatabaseWriteError("must not transmit".to_string()),
            settings,
        )),
    ];
    for result in errors {
        let error = result.unwrap_err();
        assert!(error.downcast_ref::<CloudServicesUnavailable>().is_some());
    }
    assert_eq!(std::fs::read(&path).unwrap(), original);
}

#[test]
fn disabled_collector_lifecycle_needs_no_auth_settings_or_execution_globals() {
    App::test((), |mut app| async move {
        let collector = app.add_model(|_| TelemetryCollector::new(None));
        collector.update(&mut app, |collector, ctx| {
            collector.initialize_telemetry_collection(ctx);
            collector.write_telemetry_events_to_disk(ctx);
            collector.flush_telemetry_events_for_shutdown(ctx);
        });
    });
}

use super::{
    AIConversationId, InputConfig, InputType, LongRunningCommandControlState, UserTakeOverReason,
};

#[test]
fn persisted_conversation_id_keeps_its_uuid_string_format() {
    let persisted = "\"de305d54-75b4-431b-adb2-eb6b9e546014\"";
    let id: AIConversationId = serde_json::from_str(persisted).unwrap();

    assert_eq!(serde_json::to_string(&id).unwrap(), persisted);
}

#[test]
fn legacy_stop_metadata_stays_non_resumable() {
    let state: LongRunningCommandControlState =
        serde_json::from_str(r#"{"User":{"reason":"Stop"}}"#).unwrap();

    assert_eq!(
        state,
        LongRunningCommandControlState::User {
            reason: UserTakeOverReason::Stop {
                should_auto_resume: false,
            },
        }
    );
    assert!(!state.should_auto_resume());
}

#[test]
fn resumable_stop_metadata_keeps_its_explicit_flag() {
    let persisted = r#"{"User":{"reason":{"Stop":{"should_auto_resume":true}}}}"#;
    let state: LongRunningCommandControlState = serde_json::from_str(persisted).unwrap();

    assert!(state.should_auto_resume());
    assert_eq!(serde_json::to_string(&state).unwrap(), persisted);
}

#[test]
fn shell_input_snapshot_preserves_its_locked_state() {
    let persisted = r#"{"input_type":"Shell","is_locked":true}"#;
    let config: InputConfig = serde_json::from_str(persisted).unwrap();

    assert_eq!(config.input_type, InputType::Shell);
    assert!(config.is_locked);
    assert_eq!(serde_json::to_string(&config).unwrap(), persisted);
}

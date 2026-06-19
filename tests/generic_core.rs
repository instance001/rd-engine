use std::time::{SystemTime, UNIX_EPOCH};

use rd_engine::{
    EngineAction, EngineEvent, EngineOutcome, EngineSaveEnvelope, EngineState, Reducer,
    apply_action, load_state_from_path, save_state_to_path,
};
use serde_json::{Value, json};

struct MemoryReducer;

impl Reducer for MemoryReducer {
    fn apply(
        &self,
        state: &mut EngineState,
        action: &EngineAction,
    ) -> Result<EngineOutcome, String> {
        match action.kind.as_str() {
            "remember" => {
                let bucket = action
                    .target
                    .as_deref()
                    .ok_or("remember requires a target bucket")?;
                let value = action
                    .payload
                    .get("value")
                    .and_then(Value::as_str)
                    .ok_or("remember requires payload.value")?;

                state.ensure_array_bucket(bucket).push(Value::String(value.to_owned()));

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("remembered").with_bucket(bucket)],
                    lines: vec![format!("Stored value in '{}'.", bucket)],
                })
            }
            "forget" => {
                let bucket = action
                    .target
                    .as_deref()
                    .ok_or("forget requires a target bucket")?;
                let index = action
                    .payload
                    .get("index")
                    .and_then(Value::as_u64)
                    .ok_or("forget requires payload.index")? as usize;

                let entries = state.ensure_array_bucket(bucket);
                if index >= entries.len() {
                    return Err(format!("Index {} was out of bounds for '{}'.", index, bucket));
                }
                entries.remove(index);

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("forgotten").with_bucket(bucket)],
                    lines: vec![format!("Removed entry {} from '{}'.", index, bucket)],
                })
            }
            _ => Err(format!("Unknown action '{}'.", action.kind)),
        }
    }
}

struct WorkflowReducer;

impl Reducer for WorkflowReducer {
    fn apply(
        &self,
        state: &mut EngineState,
        action: &EngineAction,
    ) -> Result<EngineOutcome, String> {
        match action.kind.as_str() {
            "set_stage" => {
                let stage = action
                    .payload
                    .get("stage")
                    .and_then(Value::as_str)
                    .ok_or("set_stage requires payload.stage")?;
                state.set_bucket("current_stage", Value::String(stage.to_owned()));

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("stage_set").with_bucket("current_stage")],
                    lines: vec![format!("Current stage is now '{}'.", stage)],
                })
            }
            "add_blocker" => {
                let blocker = action
                    .payload
                    .get("value")
                    .and_then(Value::as_str)
                    .ok_or("add_blocker requires payload.value")?;
                state
                    .ensure_array_bucket("blockers")
                    .push(Value::String(blocker.to_owned()));

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("blocker_added").with_bucket("blockers")],
                    lines: vec![format!("Added blocker '{}'.", blocker)],
                })
            }
            _ => Err(format!("Unknown action '{}'.", action.kind)),
        }
    }
}

#[test]
fn memory_style_reducer_updates_named_bucket() {
    let reducer = MemoryReducer;
    let mut state = EngineState::new();

    let outcome = apply_action(
        &reducer,
        &mut state,
        EngineAction::new("remember")
            .with_target("active_projects")
            .with_payload(json!({ "value": "Shrink RD Engine to the core pattern." })),
    );

    assert!(matches!(
        outcome.events.first(),
        Some(EngineEvent {
            kind,
            bucket: Some(bucket),
            ..
        }) if kind == "remembered" && bucket == "active_projects"
    ));
    assert_eq!(
        state.bucket("active_projects"),
        Some(&json!(["Shrink RD Engine to the core pattern."]))
    );
}

#[test]
fn workflow_style_reducer_can_move_stage_and_add_blocker() {
    let reducer = WorkflowReducer;
    let mut state = EngineState::new();

    apply_action(
        &reducer,
        &mut state,
        EngineAction::new("set_stage").with_payload(json!({ "stage": "review" })),
    );
    apply_action(
        &reducer,
        &mut state,
        EngineAction::new("add_blocker").with_payload(json!({ "value": "Need stronger docs" })),
    );

    assert_eq!(state.bucket("current_stage"), Some(&json!("review")));
    assert_eq!(state.bucket("blockers"), Some(&json!(["Need stronger docs"])));
}

#[test]
fn save_roundtrip_preserves_generic_state() {
    let mut state = EngineState::new();
    state.insert_template("shape", json!({ "notes": "array" }));
    state.set_bucket("notes", json!(["one", "two"]));

    let mut path = std::env::temp_dir();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    path.push(format!("rd_engine_generic_state_{nonce}.json"));

    let payload = EngineSaveEnvelope {
        save_version: 1,
        host_id: "generic-test-host".to_owned(),
        notes: vec!["roundtrip".to_owned()],
        diagnostic_events: vec![EngineEvent::new("saved")],
        state: state.clone(),
    };

    save_state_to_path(&path, &payload).expect("save should succeed");
    let restored = load_state_from_path(&path).expect("load should succeed");
    std::fs::remove_file(&path).ok();

    assert_eq!(restored.host_id, payload.host_id);
    assert_eq!(restored.notes, payload.notes);
    assert_eq!(restored.state.templates, state.templates);
    assert_eq!(restored.state.buckets, state.buckets);
}

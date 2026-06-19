use rd_engine::{EngineAction, EngineEvent, EngineOutcome, EngineState, Reducer, ReducerExt};
use serde_json::{Value, json};

struct ProjectMemoryReducer;

impl Reducer for ProjectMemoryReducer {
    fn apply(
        &self,
        state: &mut EngineState,
        action: &EngineAction,
    ) -> Result<EngineOutcome, String> {
        match action.kind.as_str() {
            "remember_project" => {
                let name = action
                    .payload
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or("remember_project requires payload.name")?;
                let note = action
                    .payload
                    .get("note")
                    .and_then(Value::as_str)
                    .ok_or("remember_project requires payload.note")?;

                state.object("project_memory").insert(
                    name.to_owned(),
                    json!({
                        "note": note,
                        "status": "active"
                    }),
                );

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("project_remembered").with_bucket("project_memory")],
                    lines: vec![format!("Stored project note for '{}'.", name)],
                })
            }
            "mark_project_stale" => {
                let name = action
                    .payload
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or("mark_project_stale requires payload.name")?;

                let project_memory = state.object("project_memory");
                let project = project_memory
                    .get_mut(name)
                    .and_then(Value::as_object_mut)
                    .ok_or("project did not resolve")?;
                project.insert("status".to_owned(), Value::String("stale".to_owned()));

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("project_marked_stale").with_bucket("project_memory")],
                    lines: vec![format!("Marked '{}' as stale.", name)],
                })
            }
            _ => Err(format!("Unknown action '{}'.", action.kind)),
        }
    }
}

fn main() {
    let reducer = ProjectMemoryReducer;
    let mut state = EngineState::new();

    reducer.run(
        &mut state,
        EngineAction::new("remember_project").carrying(json!({
            "name": "semantic-signal-alphabet",
            "note": "Defines model-native bucket maps for downstream adapters."
        })),
    );

    reducer.run(
        &mut state,
        EngineAction::new("mark_project_stale").carrying(json!({
            "name": "semantic-signal-alphabet"
        })),
    );

    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}

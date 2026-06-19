use rd_engine::{EngineAction, EngineEvent, EngineOutcome, EngineState, Reducer, apply_action};
use serde_json::{Value, json};

struct TodoReducer;

impl Reducer for TodoReducer {
    fn apply(
        &self,
        state: &mut EngineState,
        action: &EngineAction,
    ) -> Result<EngineOutcome, String> {
        match action.kind.as_str() {
            "add_task" => {
                let title = action
                    .payload
                    .get("title")
                    .and_then(Value::as_str)
                    .ok_or("add_task requires payload.title")?;

                let tasks = state.ensure_array_bucket("tasks");
                tasks.push(json!({ "title": title, "done": false }));

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("task_added").with_bucket("tasks")],
                    lines: vec![format!("Added task '{}'.", title)],
                })
            }
            "complete_task" => {
                let index = action
                    .payload
                    .get("index")
                    .and_then(Value::as_u64)
                    .ok_or("complete_task requires payload.index")? as usize;

                let tasks = state.ensure_array_bucket("tasks");
                let task = tasks
                    .get_mut(index)
                    .and_then(Value::as_object_mut)
                    .ok_or("task index did not resolve")?;
                task.insert("done".to_owned(), Value::Bool(true));

                Ok(EngineOutcome {
                    events: vec![EngineEvent::new("task_completed").with_bucket("tasks")],
                    lines: vec![format!("Completed task {}.", index)],
                })
            }
            _ => Err(format!("Unknown action '{}'.", action.kind)),
        }
    }
}

fn main() {
    let reducer = TodoReducer;
    let mut state = EngineState::new();

    let add = EngineAction::new("add_task").with_payload(json!({
        "title": "Ship a tiny reducer core"
    }));
    let complete = EngineAction::new("complete_task").with_payload(json!({
        "index": 0
    }));

    for action in [add, complete] {
        let outcome = apply_action(&reducer, &mut state, action);
        for line in outcome.lines {
            println!("{}", line);
        }
    }

    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}

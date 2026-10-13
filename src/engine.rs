// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type TemplateMap = BTreeMap<String, Value>;
pub type BucketMap = BTreeMap<String, Value>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EngineState {
    #[serde(default)]
    pub templates: TemplateMap,
    #[serde(default)]
    pub buckets: BucketMap,
    #[serde(default)]
    pub log: Vec<String>,
}

impl EngineState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_templates(mut self, templates: TemplateMap) -> Self {
        self.templates = templates;
        self
    }

    pub fn with_buckets(mut self, buckets: BucketMap) -> Self {
        self.buckets = buckets;
        self
    }

    pub fn insert_template(&mut self, key: impl Into<String>, value: Value) {
        self.templates.insert(key.into(), value);
    }

    pub fn set_bucket(&mut self, key: impl Into<String>, value: Value) {
        self.buckets.insert(key.into(), value);
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.set_bucket(key, value);
    }

    pub fn bucket(&self, key: &str) -> Option<&Value> {
        self.buckets.get(key)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.bucket(key)
    }

    pub fn bucket_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.buckets.get_mut(key)
    }

    pub fn ensure_object_bucket(&mut self, key: &str) -> &mut Map<String, Value> {
        let value = self
            .buckets
            .entry(key.to_owned())
            .or_insert_with(|| Value::Object(Map::new()));

        if !value.is_object() {
            *value = Value::Object(Map::new());
        }

        value
            .as_object_mut()
            .expect("bucket was converted into an object")
    }

    pub fn ensure_array_bucket(&mut self, key: &str) -> &mut Vec<Value> {
        let value = self
            .buckets
            .entry(key.to_owned())
            .or_insert_with(|| Value::Array(Vec::new()));

        if !value.is_array() {
            *value = Value::Array(Vec::new());
        }

        value
            .as_array_mut()
            .expect("bucket was converted into an array")
    }

    pub fn object(&mut self, key: &str) -> &mut Map<String, Value> {
        self.ensure_object_bucket(key)
    }

    pub fn array(&mut self, key: &str) -> &mut Vec<Value> {
        self.ensure_array_bucket(key)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineAction {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default)]
    pub payload: Value,
}

impl EngineAction {
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            target: None,
            payload: Value::Null,
        }
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn with_payload(mut self, payload: Value) -> Self {
        self.payload = payload;
        self
    }

    pub fn targeting(self, target: impl Into<String>) -> Self {
        self.with_target(target)
    }

    pub fn carrying(self, payload: Value) -> Self {
        self.with_payload(payload)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineEvent {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl EngineEvent {
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            bucket: None,
            detail: None,
        }
    }

    pub fn with_bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = Some(bucket.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct EngineOutcome {
    pub events: Vec<EngineEvent>,
    pub lines: Vec<String>,
}

pub trait Reducer {
    fn apply(
        &self,
        state: &mut EngineState,
        action: &EngineAction,
    ) -> Result<EngineOutcome, String>;
}

pub trait ReducerExt: Reducer {
    fn run(&self, state: &mut EngineState, action: EngineAction) -> EngineOutcome
    where
        Self: Sized,
    {
        apply_action(self, state, action)
    }
}

impl<T: Reducer> ReducerExt for T {}

pub fn apply_action<R: Reducer>(
    reducer: &R,
    state: &mut EngineState,
    action: EngineAction,
) -> EngineOutcome {
    let outcome = match reducer.apply(state, &action) {
        Ok(outcome) => outcome,
        Err(reason) => EngineOutcome {
            events: vec![EngineEvent::new("action_rejected")
                .with_detail(format!("{}: {}", action.kind, reason))],
            lines: vec![reason],
        },
    };

    if outcome.events.is_empty() && outcome.lines.is_empty() {
        state
            .log
            .push(format!("Action '{}' completed with no emitted detail.", action.kind));
    } else {
        state.log.extend(summarize_outcome(&action, &outcome));
    }

    outcome
}

fn summarize_outcome(action: &EngineAction, outcome: &EngineOutcome) -> Vec<String> {
    let mut lines = outcome
        .events
        .iter()
        .map(|event| {
            let mut line = format!("{} -> {}", action.kind, event.kind);
            if let Some(bucket) = &event.bucket {
                line.push_str(&format!(" [{}]", bucket));
            }
            if let Some(detail) = &event.detail {
                line.push_str(&format!(": {}", detail));
            }
            line
        })
        .collect::<Vec<_>>();

    if lines.is_empty() {
        lines.extend(
            outcome
                .lines
                .iter()
                .map(|line| format!("{} -> {}", action.kind, line)),
        );
    }

    lines
}

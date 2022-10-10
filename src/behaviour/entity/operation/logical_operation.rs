use crate::model::ReactivePropertyContainer;
use crate::reactive::BehaviourCreationError;
use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::operation::function::LogicalOperationFunction;
use crate::behaviour::entity::operation::properties::LogicalOperationProperties;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

/// Generic implementation of logical operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct LogicalOperation<'a> {
    pub f: LogicalOperationFunction,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl LogicalOperation<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: LogicalOperationFunction) -> Result<LogicalOperation<'static>, BehaviourCreationError> {
        if !e.has_property(LogicalOperationProperties::LHS.as_ref()) || !e.has_property(LogicalOperationProperties::RESULT.as_ref()) {
            return Err(BehaviourCreationError);
        }

        let internal_result = e
            .properties
            .get(LogicalOperationProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| json!(f(v.as_bool().unwrap())));

        let handle_id = Uuid::new_v4().as_u128();

        let logical_operation = LogicalOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        if let Some(lhs) = e.as_bool(LogicalOperationProperties::LHS.as_ref()) {
            e.set(LogicalOperationProperties::RESULT, Value::Bool(f(lhs)));
        }

        // Connect the internal result with the stream of the result property
        logical_operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of logical gate: {}", v);
                e.set(LogicalOperationProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        Ok(logical_operation)
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for LogicalOperation<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for LogicalOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(LogicalOperationProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(LogicalOperationProperties::RESULT.as_ref()).unwrap()
    }
}

impl Drop for LogicalOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::operation::properties::ArithmeticOperationProperties;
use crate::behaviour::entity::operation::ArithmeticOperationFunction;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const ARITHMETIC_OPERATION: &str = "arithmetic_operation";

/// Generic implementation of arithmetic operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct ArithmeticOperation<'a> {
    pub f: ArithmeticOperationFunction<f64>,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ArithmeticOperation<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: ArithmeticOperationFunction<f64>) -> Result<ArithmeticOperation<'static>, BehaviourCreationError> {
        let lhs = e.properties.get(ArithmeticOperationProperties::LHS.as_ref()).ok_or(BehaviourCreationError)?;
        let result = e.properties.get(ArithmeticOperationProperties::RESULT.as_ref()).ok_or(BehaviourCreationError)?;

        let internal_result = lhs.stream.read().unwrap().map(move |v| match v.as_f64() {
            Some(v) => json!(f(v)),
            None => json!(ArithmeticOperationProperties::RESULT.default_value()),
        });

        let handle_id = Uuid::new_v4().as_u128();

        let arithmetic_operation = ArithmeticOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        if let Some(lhs) = lhs.as_f64() {
            result.set(json!(f(lhs)));
        }

        // Connect the internal result with the stream of the result property
        let entity = e.clone();
        arithmetic_operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of {}: {}", ARITHMETIC_OPERATION, v);
                entity.set(ArithmeticOperationProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        Ok(arithmetic_operation)
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArithmeticOperation<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for ArithmeticOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(ArithmeticOperationProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(ArithmeticOperationProperties::RESULT.as_ref()).unwrap()
    }
}

impl Drop for ArithmeticOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

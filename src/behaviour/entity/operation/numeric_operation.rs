use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::behaviour::entity::operation::NumericOperationFunction;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;
use crate::NumericOperationProperties;

pub const NUMERIC_OPERATION: &str = "numeric_operation";

/// Generic implementation of numeric operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct NumericOperation<'a> {
    pub f: NumericOperationFunction<f64>,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl NumericOperation<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: NumericOperationFunction<f64>) -> NumericOperation<'static> {
        let handle_id = Uuid::new_v4().as_u128();

        let internal_result = e
            .properties
            .get(NumericOperationProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| match v.as_f64() {
                Some(v) => json!(f(v)),
                None => json!(0.0),
            });
        let numeric_operation = NumericOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Initial calculation
        let lhs_initial = e
            .as_f64(NumericOperationProperties::LHS.as_ref())
            .unwrap_or_else(|| NumericOperationProperties::LHS.default_value());
        e.set(NumericOperationProperties::RESULT.as_ref(), json!(f(lhs_initial)));

        // Connect the internal result with the stream of the result property
        numeric_operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of {}: {}", NUMERIC_OPERATION, v);
                e.set(NumericOperationProperties::RESULT.as_ref(), json!(*v));
            },
            handle_id,
        );

        numeric_operation
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for NumericOperation<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for NumericOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(NumericOperationProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(NumericOperationProperties::RESULT.as_ref()).unwrap()
    }
}

impl Drop for NumericOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

use std::convert::AsRef;
use std::sync::{Arc, RwLock};

use log::debug;
use serde_json::{json, Value};

use crate::behaviour::entity::operation::properties::ArithmeticOperationProperties;
use crate::behaviour::entity::operation::ArithmeticOperationFunction;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

pub const ARITHMETIC_OPERATION: &'static str = "arithmetic_operation";

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
    pub fn new<'a>(
        e: Arc<ReactiveEntityInstance>,
        f: ArithmeticOperationFunction<f64>,
    ) -> ArithmeticOperation<'static> {
        let handle_id = e
            .properties
            .get(ArithmeticOperationProperties::RESULT.as_ref())
            .unwrap()
            .id
            .as_u128();

        let internal_result = e
            .properties
            .get(ArithmeticOperationProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| json!(f(v.as_f64().unwrap())));
        let arithmetic_operation = ArithmeticOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        arithmetic_operation
            .internal_result
            .read()
            .unwrap()
            .observe_with_handle(
                move |v| {
                    debug!("Setting result of {}: {}", ARITHMETIC_OPERATION, v);
                    e.set(ArithmeticOperationProperties::RESULT.to_string(), json!(*v));
                },
                handle_id,
            );

        arithmetic_operation
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ArithmeticOperation<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!(
            "Disconnect {} {} with handle {}",
            ARITHMETIC_OPERATION,
            self.type_name(),
            self.handle_id
        );
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for ArithmeticOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity
            .set(ArithmeticOperationProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity
            .get(ArithmeticOperationProperties::RESULT.as_ref())
            .unwrap()
            .clone()
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ArithmeticOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

use std::convert::AsRef;
use std::sync::{Arc, RwLock};

use crate::behaviour::entity::operation::string_operation_properties::StringOperationProperties;
use log::debug;
use serde_json::{json, Value};

use crate::behaviour::entity::operation::StringOperationFunction;
use crate::frp::Stream;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

/// Generic implementation of string operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct StringOperation<'a> {
    pub f: StringOperationFunction,

    pub internal_result: RwLock<Stream<'a, Value>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StringOperation<'_> {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>, f: StringOperationFunction) -> StringOperation<'static> {
        let handle_id = e.properties.get(StringOperationProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let internal_result = e
            .properties
            .get(StringOperationProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| json!(f(v.as_str().unwrap().into())));
        let string_operation = StringOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        string_operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of string gate: {}", v);
                e.set(StringOperationProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        string_operation
    }

    /// TODO: extract to trait "Named"
    /// TODO: unit test
    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StringOperation<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect string operation {}", self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StringOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StringOperationProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StringOperationProperties::RESULT.as_ref()).unwrap().clone()
    }
}

/// Automatically disconnect streams on destruction
impl Drop for StringOperation<'_> {
    fn drop(&mut self) {
        debug!("Drop string operation");
        self.disconnect();
    }
}

use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::operation::properties::StringOperationProperties;
use crate::behaviour::entity::operation::StringOperationFunction;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
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
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StringOperationFunction) -> StringOperation<'static> {
        let handle_id = e.properties.get(StringOperationProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let internal_result = e
            .properties
            .get(StringOperationProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| match v.as_str() {
                Some(v) => json!(f(v.into())),
                None => StringOperationProperties::RESULT.default_value(),
            });
        let operation = StringOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                debug!("Setting result of string gate: {}", v);
                e.set(StringOperationProperties::RESULT.to_string(), v.clone()); // json!(*v)
            },
            handle_id,
        );

        operation
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StringOperation<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StringOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StringOperationProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StringOperationProperties::RESULT.as_ref()).unwrap()
    }
}

impl Drop for StringOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

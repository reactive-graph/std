use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::str_bool_operation::StrBoolFunction;
use crate::behaviour::entity::str_bool_operation::StrBoolProperties;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

/// Generic implementation of string operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct StrBoolOperation<'a> {
    pub f: StrBoolFunction,

    pub internal_result: RwLock<Stream<'a, bool>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StrBoolOperation<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StrBoolFunction) -> StrBoolOperation<'static> {
        let handle_id = e.properties.get(StrBoolProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let internal_result = e
            .properties
            .get(StrBoolProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| match v.as_str() {
                Some(v) => f(v.into()),
                None => StrBoolProperties::RESULT.default_value().as_bool().unwrap(),
            });
        let operation = StrBoolOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(StrBoolProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        operation
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StrBoolOperation<'_> {
    /// TODO: Add guard: disconnect only if actually connected
    fn disconnect(&self) {
        debug!("Disconnect string operation {}", self.handle_id);
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StrBoolOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StrBoolProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StrBoolProperties::RESULT.as_ref()).unwrap()
    }
}

impl Drop for StrBoolOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

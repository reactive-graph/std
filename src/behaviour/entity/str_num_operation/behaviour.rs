use std::convert::AsRef;
use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::str_num_operation::StrNumFunction;
use crate::behaviour::entity::str_num_operation::StrNumProperties;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

/// Generic implementation of string operations with one input and one result.
///
/// The implementation is realized using reactive streams.
pub struct StrNumOperation<'a> {
    pub f: StrNumFunction,

    pub internal_result: RwLock<Stream<'a, usize>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StrNumOperation<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StrNumFunction) -> StrNumOperation<'static> {
        let handle_id = e.properties.get(StrNumProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let internal_result = e
            .properties
            .get(StrNumProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(move |v| match v.as_str() {
                Some(v) => f(v.into()),
                None => 0,
            });
        let string_operation = StrNumOperation {
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        string_operation.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(StrNumProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        string_operation
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StrNumOperation<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StrNumOperation<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StrNumProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StrNumProperties::RESULT.as_ref()).unwrap()
    }
}

impl Drop for StrNumOperation<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

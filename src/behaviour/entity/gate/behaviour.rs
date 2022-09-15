use std::sync::Arc;
use std::sync::RwLock;

use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::gate::function::StringGateFunction;
use crate::behaviour::entity::gate::properties::StringGateProperties;
use crate::frp::Stream;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::expression::Expression;
use crate::reactive::entity::expression::ExpressionValue;
use crate::reactive::entity::expression::OperatorPosition;
use crate::reactive::entity::gate::Gate;
use crate::reactive::entity::operation::Operation;
use crate::reactive::entity::Disconnectable;

pub type StringExpressionValue = ExpressionValue<String>;

/// Generic implementation of string_gates operations with two inputs (LHS,RHS) and one result.
///
/// The implementation is realized using reactive streams.
pub struct StringGate<'a> {
    pub lhs: RwLock<Stream<'a, StringExpressionValue>>,

    pub rhs: RwLock<Stream<'a, StringExpressionValue>>,

    pub f: StringGateFunction,

    pub internal_result: RwLock<Stream<'a, String>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StringGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StringGateFunction) -> StringGate<'static> {
        let lhs = e
            .properties
            .get(StringGateProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| match v.as_str() {
                Some(lhs_str) => (OperatorPosition::LHS, String::from(lhs_str)),
                None => (OperatorPosition::LHS, StringGateProperties::LHS.default_value()),
            });
        let rhs = e
            .properties
            .get(StringGateProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> StringExpressionValue {
                match v.as_str() {
                    Some(rhs_str) => (OperatorPosition::RHS, String::from(rhs_str)),
                    None => (OperatorPosition::RHS, StringGateProperties::RHS.default_value()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(StringGateProperties::LHS.default_value(), StringGateProperties::RHS.default_value()),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs.clone(), e.rhs.clone()));

        let handle_id = e.properties.get(StringGateProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let string_gate = StringGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        string_gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(StringGateProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        string_gate
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StringGate<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StringGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StringGateProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StringGateProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for StringGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(StringGateProperties::RHS.as_ref(), value);
    }
}

impl Drop for StringGate<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

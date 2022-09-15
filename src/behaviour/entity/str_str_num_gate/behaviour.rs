use std::sync::Arc;
use std::sync::RwLock;

use log::debug;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::str_str_num_gate::StrStrNumFunction;
use crate::behaviour::entity::str_str_num_gate::StrStrNumProperties;
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
pub struct StrStrNumGate<'a> {
    pub lhs: RwLock<Stream<'a, StringExpressionValue>>,

    pub rhs: RwLock<Stream<'a, StringExpressionValue>>,

    pub f: StrStrNumFunction,

    pub internal_result: RwLock<Stream<'a, usize>>,

    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl StrStrNumGate<'_> {
    pub fn new(e: Arc<ReactiveEntityInstance>, f: StrStrNumFunction) -> StrStrNumGate<'static> {
        let lhs = e
            .properties
            .get(StrStrNumProperties::LHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> StringExpressionValue {
                match v.as_str() {
                    Some(lhs_str) => (OperatorPosition::LHS, String::from(lhs_str)),
                    None => (OperatorPosition::LHS, StrStrNumProperties::LHS.default_value().as_str().unwrap().into()),
                }
            });
        let rhs = e
            .properties
            .get(StrStrNumProperties::RHS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .map(|v| -> StringExpressionValue {
                match v.as_str() {
                    Some(rhs_str) => (OperatorPosition::RHS, String::from(rhs_str)),
                    None => (OperatorPosition::RHS, StrStrNumProperties::RHS.default_value().as_str().unwrap().into()),
                }
            });

        let expression = lhs.merge(&rhs).fold(
            Expression::new(
                StrStrNumProperties::LHS.default_value().as_str().unwrap().into(),
                StrStrNumProperties::RHS.default_value().as_str().unwrap().into(),
            ),
            |old_state, (o, value)| match *o {
                OperatorPosition::LHS => old_state.lhs(value.clone()),
                OperatorPosition::RHS => old_state.rhs(value.clone()),
            },
        );

        // The internal result
        let internal_result = expression.map(move |e| f(e.lhs.clone(), e.rhs.clone()));

        let handle_id = e.properties.get(StrStrNumProperties::RESULT.as_ref()).unwrap().id.as_u128();

        let gate = StrStrNumGate {
            lhs: RwLock::new(lhs),
            rhs: RwLock::new(rhs),
            f,
            internal_result: RwLock::new(internal_result),
            entity: e.clone(),
            handle_id,
        };

        // Connect the internal result with the stream of the result property
        gate.internal_result.read().unwrap().observe_with_handle(
            move |v| {
                e.set(StrStrNumProperties::RESULT.to_string(), json!(*v));
            },
            handle_id,
        );

        gate
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for StrStrNumGate<'_> {
    fn disconnect(&self) {
        self.internal_result.read().unwrap().remove(self.handle_id);
    }
}

impl Operation for StrStrNumGate<'_> {
    fn lhs(&self, value: Value) {
        self.entity.set(StrStrNumProperties::LHS.as_ref(), value);
    }

    fn result(&self) -> Value {
        self.entity.get(StrStrNumProperties::RESULT.as_ref()).unwrap()
    }
}

impl Gate for StrStrNumGate<'_> {
    fn rhs(&self, value: Value) {
        self.entity.set(StrStrNumProperties::RHS.as_ref(), value);
    }
}

impl Drop for StrStrNumGate<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
